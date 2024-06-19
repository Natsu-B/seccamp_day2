#![no_std]
#![no_main]

const PL011: usize = 0x09000000;

#[no_mangle]
extern "efiapi" fn efi_main(_main_handle: usize, _system_table: usize) -> ! {
    let text = "Hello,world!\nLet's make a hypervisor!!\n";

    for c in text.as_bytes() {
        putc(*c);
    }

    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

fn putc(c: u8) {
    //uratでの送信を行う関数をつくる 受信は考えないでやるよ ASCIIで送受信することにする
    //qemuは-nographicがついているとUARTからよみだそうとするらしい? あとPL011は単にバッファに書き込むだけで開始、終了コマンドを勝手によしなにしてくれるらしい
    unsafe {
        core::ptr::write_volatile(PL011 as *mut u8, c);//volatileはコンパイラがforループで何も使われてないのに無駄に書き込みが起こっていると考えて最適化で消さないようにするためについてる
    }
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}