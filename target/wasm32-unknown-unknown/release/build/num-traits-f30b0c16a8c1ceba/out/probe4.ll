; ModuleID = 'probe4.19a32b8006b717d7-cgu.0'
source_filename = "probe4.19a32b8006b717d7-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_5f53a383c894c5ae722f760bee094514 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/79e9716c980570bfd1f666e3b16ac583f0168962/library/core/src/num/mod.rs" }>, align 1
@alloc_ce45336a62f4a128ef8d30d5310b424b = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_5f53a383c894c5ae722f760bee094514, [12 x i8] c"K\00\00\00v\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe45probe17h700942ab99476e80E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h1adca2f269a84f04E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h7bbea3773b752235E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_ce45336a62f4a128ef8d30d5310b424b) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h1adca2f269a84f04E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h7bbea3773b752235E(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" }
attributes #3 = { noreturn nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.74.0 (79e9716c9 2023-11-13)"}
