; ModuleID = 'sum'
source_filename = "sum"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

define i64 @sum(i64 %0, i64 %1, i64 %2) {
entry:
  %sum = add i64 %0, %1
  %sum1 = add i64 %sum, %2
  ret i64 %sum1
}

