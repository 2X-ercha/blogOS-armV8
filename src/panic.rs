use core::panic::PanicInfo;

// 发散函数（diverging function）是rust中的一个特性。发散函数不返回，它使用感叹号!作为返回类型表示
/*
    在Rust中，有这些情况永远不会返回，它们的类型就是！，它们是：
    1，panic！以及基于它实现的各种函数/宏，比如unimplemented！、 unreachable！；
    2，无限循环loop{}；
    3，进程退出函数std::process::exit以及类似的libc中的exec一类函数。
*/
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}
