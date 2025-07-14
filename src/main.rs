use windows::{
    core::PCSTR,
    Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress},
    Win32::Foundation::{FreeLibrary},
    Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS}
};
use std::ptr;
fn cleanup(){
    unsafe {
        let dll_name = format!("ntdll.dll");
        let handle_dll = match LoadLibraryA(PCSTR(dll_name.as_ptr())) {
            Ok(handle) => handle,
            Err(err) => {
                println!("Error loading dll: {}", err);
                return;
            }
        };

        let func_name = format!("NtTraceEvent");

        let trace_event = GetProcAddress(handle_dll, PCSTR(func_name.as_ptr()));

        match trace_event {
            Some(ptr) => {
                println!("NtTraceEvent is at {:p}", ptr);
            },
            None => {
                println!("NtTraceEvent is not found");
            }
        }

        //let addr: *mut u8 = trace_event.map_or(std::ptr::null_mut(), |f| f as *mut u8);
        //if !addr.is_null() {
            //patch_hook(addr);
        //}
        
        //patch_hook(trace_event as *mut u8);
        if let Some(ptr) = trace_event {
            let ptr = ptr as *const () as *mut u8;
            patch_hook(ptr);
        }

        match FreeLibrary(handle_dll) {
            Ok(_) => println!("Dll freed"),
            Err(err) => println!("Error freeing dll: {}", err)
        }
    }

}


fn patch_hook(address: *mut u8){
    unsafe{
        let mut old_protect = PAGE_PROTECTION_FLAGS(0);

        if let Ok(_) = VirtualProtect(
            address.add(3) as _,
            1,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect
        ) {
            println!("\nOld permission 0x{:08x}", old_protect.0);

            ptr::write(address.add(3), 0xc3);

            let mut _temp = PAGE_PROTECTION_FLAGS(0);
            match VirtualProtect(
                address.add(3) as _,
                1,
                old_protect,
                &mut _temp
            ) {
                Ok(_) => println!("New permission 0x{:08x}", old_protect.0),
                Err(err) => println!("Error: {}", err)
            }
        }
    }

}


fn main(){
    cleanup();
    
    // do something
}