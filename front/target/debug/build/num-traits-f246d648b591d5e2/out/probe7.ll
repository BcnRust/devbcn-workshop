; ModuleID = 'probe7.d20f2d5f-cgu.0'
source_filename = "probe7.d20f2d5f-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; std::f64::<impl f64>::copysign
; Function Attrs: inlinehint nonlazybind uwtable
define internal double @"_ZN3std3f6421_$LT$impl$u20$f64$GT$8copysign17h608d17b3110aa0b6E"(double %self, double %sign) unnamed_addr #0 {
start:
  %0 = alloca double, align 8
  %1 = call double @llvm.copysign.f64(double %self, double %sign)
  store double %1, ptr %0, align 8
  %2 = load double, ptr %0, align 8, !noundef !2
  ret double %2
}

; probe7::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe75probe17h0ad07eded71f147dE() unnamed_addr #1 {
start:
; call std::f64::<impl f64>::copysign
  %_1 = call double @"_ZN3std3f6421_$LT$impl$u20$f64$GT$8copysign17h608d17b3110aa0b6E"(double 1.000000e+00, double -1.000000e+00)
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare double @llvm.copysign.f64(double, double) #2

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{}
