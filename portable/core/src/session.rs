//! A bounded owned ledger. Completion, publication and redemption are separate.
use crate::error;
use kuiper_contracts::Result;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);
const WINDOW: u32 = 64;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Accepted,
    InFlight,
    CompletedUnchecked,
    Succeeded,
    Failed,
}
impl Phase {
    fn terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Handle {
    generation: u64,
    operation: u32,
}
struct Operation {
    phase: Phase,
    dependencies: Vec<u32>,
    payload: Option<Vec<u8>>,
    redeemed: bool,
}
pub struct Session {
    generation: u64,
    retired: u32,
    next: u32,
    operations: BTreeMap<u32, Operation>,
    owner: PhantomData<Rc<()>>,
}
impl Session {
    pub(crate) fn new() -> Result<Self> {
        let generation = NEXT_GENERATION
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| error("generation-exhausted", "session identities exhausted"))?;
        Ok(Self {
            generation,
            retired: 0,
            next: 1,
            operations: BTreeMap::new(),
            owner: PhantomData,
        })
    }
    fn operation(&self, handle: Handle) -> Result<&Operation> {
        if handle.generation != self.generation || handle.operation <= self.retired {
            return Err(error(
                "stale-handle",
                "handle belongs to another or retired session",
            ));
        }
        self.operations
            .get(&handle.operation)
            .ok_or_else(|| error("unknown-handle", "operation is absent"))
    }
    fn operation_mut(&mut self, handle: Handle) -> Result<&mut Operation> {
        self.operation(handle)?;
        self.operations
            .get_mut(&handle.operation)
            .ok_or_else(|| error("unknown-handle", "operation is absent"))
    }
    pub(crate) fn accept(&mut self, payload: Vec<u8>, dependencies: &[Handle]) -> Result<Handle> {
        if self.next - self.retired > WINDOW {
            return Err(error("ledger-window", "operation window is full"));
        }
        let mut ids = Vec::new();
        for dependency in dependencies {
            self.operation(*dependency)?;
            if ids.contains(&dependency.operation) {
                return Err(error("duplicate-dependency", "dependency repeated"));
            }
            ids.push(dependency.operation);
        }
        let handle = Handle {
            generation: self.generation,
            operation: self.next,
        };
        self.next = self
            .next
            .checked_add(1)
            .ok_or_else(|| error("operation-exhausted", "operation identities exhausted"))?;
        self.operations.insert(
            handle.operation,
            Operation {
                phase: Phase::Accepted,
                dependencies: ids,
                payload: Some(payload),
                redeemed: false,
            },
        );
        Ok(handle)
    }
    pub(crate) fn phase(&self, handle: Handle) -> Result<Phase> {
        Ok(self.operation(handle)?.phase)
    }
    pub(crate) fn begin(&mut self, handle: Handle) -> Result<()> {
        let operation = self.operation(handle)?;
        if operation.phase != Phase::Accepted || operation.payload.is_none() {
            return Err(error("phase", "operation is not retained and accepted"));
        }
        for id in &operation.dependencies {
            if self
                .operations
                .get(id)
                .is_none_or(|o| o.phase != Phase::Succeeded)
            {
                return Err(error(
                    "dependency-not-successful",
                    "completion or failure cannot satisfy a dependency",
                ));
            }
        }
        self.operation_mut(handle)?.phase = Phase::InFlight;
        Ok(())
    }
    pub(crate) fn complete(&mut self, handle: Handle) -> Result<()> {
        let operation = self.operation_mut(handle)?;
        if operation.phase != Phase::InFlight {
            return Err(error("phase", "completion requires in-flight work"));
        }
        operation.phase = Phase::CompletedUnchecked;
        Ok(())
    }
    pub(crate) fn publish(&mut self, handle: Handle) -> Result<()> {
        // The sole caller has established visibility, zero guard and postcheck.
        let operation = self.operation_mut(handle)?;
        if operation.phase != Phase::CompletedUnchecked {
            return Err(error("phase", "publication requires checked completion"));
        }
        operation.phase = Phase::Succeeded;
        Ok(())
    }
    pub(crate) fn fail(&mut self, handle: Handle) -> Result<()> {
        let operation = self.operation_mut(handle)?;
        if operation.phase.terminal() {
            return Err(error("phase", "terminal result is immutable"));
        }
        operation.phase = Phase::Failed;
        Ok(())
    }
    pub(crate) fn redeem(&mut self, handle: Handle, device_quiescent: bool) -> Result<()> {
        let operation = self.operation_mut(handle)?;
        if !operation.phase.terminal() || operation.redeemed || !device_quiescent {
            return Err(error(
                "not-redeemable",
                "redemption requires an unredeemed terminal result and quiescence",
            ));
        }
        operation.redeemed = true;
        operation.payload = None;
        Ok(())
    }
    pub(crate) fn retire(&mut self, next: u32) -> Result<()> {
        if next < self.retired || next >= self.next {
            return Err(error(
                "retirement-range",
                "retirement is outside the accepted prefix",
            ));
        }
        for id in (self.retired + 1)..=next {
            if self
                .operations
                .get(&id)
                .is_none_or(|o| !o.phase.terminal() || !o.redeemed)
            {
                return Err(error(
                    "pending-retirement",
                    "prefix contains retained or unresolved work",
                ));
            }
        }
        for operation in self.operations.range((next + 1)..).map(|(_, op)| op) {
            if !operation.phase.terminal() && operation.dependencies.iter().any(|id| *id <= next) {
                return Err(error(
                    "dependency-retirement",
                    "an unresolved dependent still needs this prefix",
                ));
            }
        }
        self.operations.retain(|id, _| *id > next);
        self.retired = next;
        Ok(())
    }
    pub(crate) fn last_id(&self) -> u32 {
        self.next - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn completion_does_not_satisfy_dependency() {
        let mut session = Session::new().unwrap();
        let a = session.accept(vec![1], &[]).unwrap();
        let b = session.accept(vec![2], &[a]).unwrap();
        session.begin(a).unwrap();
        session.complete(a).unwrap();
        assert!(session.begin(b).is_err());
        session.publish(a).unwrap();
        session.begin(b).unwrap();
    }
    #[test]
    fn forgotten_handle_retains_and_redemption_needs_quiescence() {
        let mut session = Session::new().unwrap();
        let handle = session.accept(vec![1, 2, 3], &[]).unwrap();
        session.begin(handle).unwrap();
        assert_eq!(
            session.operation(handle).unwrap().payload.as_deref(),
            Some(&[1, 2, 3][..])
        );
        session.fail(handle).unwrap();
        assert!(session.redeem(handle, false).is_err());
        session.redeem(handle, true).unwrap();
        assert!(session.redeem(handle, true).is_err());
    }
    #[test]
    fn failed_dependency_and_retirement_preserve_failure() {
        let mut session = Session::new().unwrap();
        let a = session.accept(vec![], &[]).unwrap();
        let b = session.accept(vec![], &[a]).unwrap();
        session.fail(a).unwrap();
        session.redeem(a, true).unwrap();
        assert!(session.retire(a.operation).is_err());
        assert!(session.begin(b).is_err());
        session.fail(b).unwrap();
        session.redeem(b, true).unwrap();
        session.retire(b.operation).unwrap();
        assert!(session.phase(a).is_err());
    }
    #[test]
    fn identities_cannot_cross_sessions() {
        let mut a = Session::new().unwrap();
        let mut b = Session::new().unwrap();
        let handle = a.accept(vec![], &[]).unwrap();
        b.accept(vec![], &[]).unwrap();
        assert!(b.begin(handle).is_err());
    }
}
