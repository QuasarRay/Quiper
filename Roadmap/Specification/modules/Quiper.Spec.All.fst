module Quiper.Spec.All
open Quiper.Spec.Foundation
open Quiper.Spec.Kernel
open Quiper.Spec.Memory
open Quiper.Spec.Runtime
open Quiper.Spec.Refinement
open Quiper.Spec.Extension
open Quiper.Spec.Lowering
open Quiper.Spec.Qualification
open Quiper.Spec.Operations
open Quiper.Spec.Host

// This file is a dependency root for strict verification of the specification.
let specification_version : nat = 3
