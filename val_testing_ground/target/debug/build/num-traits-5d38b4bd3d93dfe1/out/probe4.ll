; ModuleID = 'probe4.ebd3d1aebc16d446-cgu.0'
source_filename = "probe4.ebd3d1aebc16d446-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@alloc_c29affd05bd08348e0cd27cc0c635618 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/7d9bce327332fef22cf687b44de58f06d30daeb0\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc_91bcfbe8d485aaa70bff897831c8cdac = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c29affd05bd08348e0cd27cc0c635618, [16 x i8] c"K\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h1d90abe9d3dbd61eE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h54f2bbb43b9f83a9E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hee6530c597330ee8E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_91bcfbe8d485aaa70bff897831c8cdac) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h54f2bbb43b9f83a9E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hee6530c597330ee8E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.74.0-nightly (7d9bce327 2023-09-16)"}
