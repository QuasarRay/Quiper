#ifndef KUIPER_PORTABLE_H
#define KUIPER_PORTABLE_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

/* v1 is synchronous. Input byte spans are copied; no caller pointer survives
 * return. required must be writable and aligned for uint64_t. With sufficient
 * output capacity, nonempty inputs must remain readable during the call and
 * output must have capacity writable bytes. All accessed spans are disjoint.
 * A capacity probe only accesses required.
 *
 * flags must be 1 (explicit experimental admission). A null output or capacity
 * below the reported bound returns CAPACITY without reading inputs or executing
 * work. First probe reports 16 MiB. Success reports the actual UTF-8 JSON length;
 * output is a byte span, not a NUL-terminated string.
 *
 * Status: 0 OK; 1 ARGUMENT; 2 CAPACITY; 3 REJECTED with diagnostic JSON;
 * 4 FAILED with host-plan result JSON; 5 PANIC. Only a zero device guard and
 * checked postcondition permit OK. Failed host versions have null words.
 */
uint32_t kuiper_run_v1(const uint8_t *root, uint64_t root_len,
    const uint8_t *package, uint64_t package_len,
    const uint8_t *invocation, uint64_t invocation_len, uint32_t flags,
    uint8_t *output, uint64_t capacity, uint64_t *required);
uint32_t kuiper_run_plan_v1(const uint8_t *root, uint64_t root_len,
    const uint8_t *package, uint64_t package_len,
    const uint8_t *plan, uint64_t plan_len, uint32_t flags,
    uint8_t *output, uint64_t capacity, uint64_t *required);
#ifdef __cplusplus
}
#endif
#endif
