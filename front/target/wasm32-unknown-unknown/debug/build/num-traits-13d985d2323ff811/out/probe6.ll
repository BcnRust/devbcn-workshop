; ModuleID = 'probe6.ccac7103-cgu.0'
source_filename = "probe6.ccac7103-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_e2f0cba8101c3fa1817c859afd61fcb8 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/num/mod.rs" }>, align 1
@alloc_5cb1768a42dd7972ed2a5f447b81b7ea = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_e2f0cba8101c3fa1817c859afd61fcb8, [12 x i8] c"K\00\00\00/\04\00\00\05\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe65probe17hfb2f8b9a466229caE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hb6eb52f71a3e1441E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hea78c4e8b318407dE(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_5cb1768a42dd7972ed2a5f447b81b7ea) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hb6eb52f71a3e1441E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare hidden i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17hea78c4e8b318407dE(ptr align 1, i32, ptr align 4) unnamed_addr #2

attributes #0 = { nounwind "target-cpu"="generic" }
attributes #1 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" }
attributes #3 = { noreturn nounwind }
