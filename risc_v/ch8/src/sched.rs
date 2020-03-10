// sched.rs
// Simple process scheduler
// Stephen Marz
// 27 Dec 2019

use crate::{process::{ProcessState, PROCESS_LIST}};

extern "C" {
	fn switch_to_user(frame: usize, mepc: usize, satp: usize) -> !;
}

pub fn schedule() {
	unsafe {
		if let Some(mut pl) = PROCESS_LIST.take() {
			pl.rotate_left(1);
			let mut frame_addr: usize = 0;
			let mut mepc: usize = 0;
			let mut satp: usize = 0;
			let mut pid: usize = 0;
			if let Some(prc) = pl.front() {
				match prc.get_state() {
					ProcessState::Running => {
						frame_addr =
							prc.get_frame_address();
						mepc = prc.get_program_counter();
						satp = prc.get_table_address() >> 12;
						pid = prc.get_pid() as usize;
					},
					ProcessState::Sleeping => {
						
					},
					_ => {},
				}
			}
			PROCESS_LIST.replace(pl);
			println!("Sched ->\n  frame = 0x{:08x}\n  mepc = 0x{:08x}\n  satp = 0x{:08x}\n  pid = {}",
				frame_addr, mepc, satp, pid
			);
			if frame_addr != 0 {
				// MODE 8 is 39-bit virtual address MMU
				// I'm using the PID as the address space identifier to hopefully
				// help with (not?) flushing the TLB whenever we switch processes.
				if satp != 0 {
					switch_to_user(frame_addr, mepc, (8 << 60) | (pid << 44) | satp);
				}
				else {
					switch_to_user(frame_addr, mepc, 0);
				}
			}
		}
	}
}
