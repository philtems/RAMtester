use std::env;
use std::io::{self, Write, Read};
use std::time::{Instant, Duration};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use rand::Rng;

// Constantes pour les messages en français
const FR_TITLE: &str = "RAMtester v3.0";
const FR_AUTHOR: &str = "2025, Philippe TEMESI";
const FR_WEBSITE: &str = "https://www.tems.be";
const FR_USAGE: &str = "Utilisation : ramtester <taille> (ex: 512M, 2G ou MAX) [--loop]";
const FR_MEMORY_ERROR: &str = "Erreur : impossible d'allouer ";
const FR_MEMORY_UNIT_ERROR: &str = "Unité invalide. Utilisez M (Méga), G (Giga) ou MAX.";
const FR_SWAP_WARNING: &str = "Attention! Quantité demandée supérieure à la mémoire disponible. Risque de SWAP!!!";
const FR_TOTAL_MEMORY: &str = "Mémoire totale du système: ";
const FR_AVAILABLE_MEMORY: &str = "Mémoire disponible : ";
const FR_FILLING_MEMORY: &str = "Remplissage de la mémoire en cours...";
const FR_VERIFYING_MEMORY: &str = "Vérification de la mémoire en cours...";
const FR_FILLING_COMPLETE: &str = "Remplissage terminé en ";
const FR_VERIFICATION_COMPLETE: &str = "Vérification terminée en ";
const FR_TEST_SUCCESS: &str = "Test réussi : aucune erreur détectée.";
const FR_TEST_FAILED: &str = "Test échoué : ";
const FR_ERROR_AT_ADDRESS: &str = "Erreur à l'adresse : ";
const FR_SUMMARY: &str = "===== Récapitulatif =====";
const FR_LOOP_COUNT: &str = "Nombre de boucles effectuées : ";
const FR_TOTAL_ERRORS: &str = "Erreurs totales détectées : ";
const FR_TOTAL_TESTED: &str = "Quantité totale de mémoire testée : ";
const FR_TOTAL_TIME: &str = "Temps total écoulé : ";
const FR_INTERRUPTED: &str = "Test interrompu par l'utilisateur.";
const FR_ERROR_MAP: &str = "Carte des erreurs mémoire";
const FR_PRESS_ESC: &str = "Appuyez sur ESC pour continuer...";
const FR_BLOCK_SIZE: &str = "1 car. = ";

// Messages en anglais
const EN_TITLE: &str = "RAMtester v3.0";
const EN_AUTHOR: &str = "2025, Philippe TEMESI";
const EN_WEBSITE: &str = "https://www.tems.be";
const EN_USAGE: &str = "Usage: ramtester <size> (ex: 512M, 2G or MAX) [--loop]";
const EN_MEMORY_ERROR: &str = "Error: unable to allocate ";
const EN_MEMORY_UNIT_ERROR: &str = "Invalid unit. Use M (Mega), G (Giga) or MAX.";
const EN_SWAP_WARNING: &str = "Warning! Requested size exceeds available memory. Risk of SWAP!!!";
const EN_TOTAL_MEMORY: &str = "Total system memory: ";
const EN_AVAILABLE_MEMORY: &str = "Available memory: ";
const EN_FILLING_MEMORY: &str = "Filling memory...";
const EN_VERIFYING_MEMORY: &str = "Verifying memory...";
const EN_FILLING_COMPLETE: &str = "Filling completed in ";
const EN_VERIFICATION_COMPLETE: &str = "Verification completed in ";
const EN_TEST_SUCCESS: &str = "Test successful: no errors detected.";
const EN_TEST_FAILED: &str = "Test failed: ";
const EN_ERROR_AT_ADDRESS: &str = "Error at address: ";
const EN_SUMMARY: &str = "===== Summary =====";
const EN_LOOP_COUNT: &str = "Number of loops completed: ";
const EN_TOTAL_ERRORS: &str = "Total errors detected: ";
const EN_TOTAL_TESTED: &str = "Total memory tested: ";
const EN_TOTAL_TIME: &str = "Total time elapsed: ";
const EN_INTERRUPTED: &str = "Test interrupted by user.";
const EN_ERROR_MAP: &str = "Memory Error Map";
const EN_PRESS_ESC: &str = "Press ESC to continue...";
const EN_BLOCK_SIZE: &str = "1 char = ";

// Color constants for ANSI terminals
#[cfg(not(target_os = "windows"))]
mod colors {
    pub const BRIGHT_WHITE: &str = "\x1b[97m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
}

// Color constants for Windows (empty strings, we'll use Windows API)
#[cfg(target_os = "windows")]
mod colors {
    pub const BRIGHT_WHITE: &str = "";
    pub const BRIGHT_CYAN: &str = "";
    pub const BRIGHT_MAGENTA: &str = "";
    pub const BRIGHT_YELLOW: &str = "";
    pub const BRIGHT_GREEN: &str = "";
    pub const BRIGHT_RED: &str = "";
    pub const BRIGHT_BLUE: &str = "";
    pub const RESET: &str = "";
    pub const BOLD: &str = "";
}

use colors::*;

// Windows console handling
#[cfg(target_os = "windows")]
mod windows_console {
    use winapi::um::wincon::SetConsoleTextAttribute;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::{
        FOREGROUND_RED, FOREGROUND_GREEN, FOREGROUND_BLUE,
        FOREGROUND_INTENSITY,
    };
    use std::io;

    pub struct ConsoleColor {
        original_attrs: u16,
    }

    impl ConsoleColor {
        pub fn new() -> io::Result<Self> {
            unsafe {
                let handle = GetStdHandle(STD_OUTPUT_HANDLE);
                if handle.is_null() {
                    return Err(io::Error::last_os_error());
                }
                
                // We can't get current attributes easily, so we'll assume default (white on black)
                // In a real implementation, you'd use GetConsoleScreenBufferInfo
                Ok(ConsoleColor { original_attrs: 7 }) // 7 = white on black
            }
        }

        pub fn set_color(&self, color: u16) {
            unsafe {
                let handle = GetStdHandle(STD_OUTPUT_HANDLE);
                if !handle.is_null() {
                    SetConsoleTextAttribute(handle, color);
                }
            }
        }

        pub fn reset(&self) {
            self.set_color(self.original_attrs);
        }
    }

    impl Drop for ConsoleColor {
        fn drop(&mut self) {
            self.reset();
        }
    }

    // Color constants for Windows console
    pub const WHITE: u16 = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY;
    pub const CYAN: u16 = FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY;
    pub const MAGENTA: u16 = FOREGROUND_RED | FOREGROUND_BLUE | FOREGROUND_INTENSITY;
    pub const YELLOW: u16 = FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY;
    pub const GREEN: u16 = FOREGROUND_GREEN | FOREGROUND_INTENSITY;
    pub const RED: u16 = FOREGROUND_RED | FOREGROUND_INTENSITY;
    pub const BLUE: u16 = FOREGROUND_BLUE | FOREGROUND_INTENSITY;
    pub const RESET: u16 = 7; // Default gray
}

struct Messages {
    title: String,
    author: String,
    website: String,
    usage: String,
    memory_error: String,
    memory_unit_error: String,
    swap_warning: String,
    total_memory: String,
    available_memory: String,
    filling_memory: String,
    verifying_memory: String,
    filling_complete: String,
    verification_complete: String,
    test_success: String,
    test_failed: String,
    error_at_address: String,
    summary: String,
    loop_count: String,
    total_errors: String,
    total_tested: String,
    total_time: String,
    interrupted: String,
    error_map: String,
    press_esc: String,
    block_size: String,
}

impl Messages {
    fn new(lang: &str) -> Self {
        if lang.starts_with("fr") {
            Messages {
                title: FR_TITLE.to_string(),
                author: FR_AUTHOR.to_string(),
                website: FR_WEBSITE.to_string(),
                usage: FR_USAGE.to_string(),
                memory_error: FR_MEMORY_ERROR.to_string(),
                memory_unit_error: FR_MEMORY_UNIT_ERROR.to_string(),
                swap_warning: FR_SWAP_WARNING.to_string(),
                total_memory: FR_TOTAL_MEMORY.to_string(),
                available_memory: FR_AVAILABLE_MEMORY.to_string(),
                filling_memory: FR_FILLING_MEMORY.to_string(),
                verifying_memory: FR_VERIFYING_MEMORY.to_string(),
                filling_complete: FR_FILLING_COMPLETE.to_string(),
                verification_complete: FR_VERIFICATION_COMPLETE.to_string(),
                test_success: FR_TEST_SUCCESS.to_string(),
                test_failed: FR_TEST_FAILED.to_string(),
                error_at_address: FR_ERROR_AT_ADDRESS.to_string(),
                summary: FR_SUMMARY.to_string(),
                loop_count: FR_LOOP_COUNT.to_string(),
                total_errors: FR_TOTAL_ERRORS.to_string(),
                total_tested: FR_TOTAL_TESTED.to_string(),
                total_time: FR_TOTAL_TIME.to_string(),
                interrupted: FR_INTERRUPTED.to_string(),
                error_map: FR_ERROR_MAP.to_string(),
                press_esc: FR_PRESS_ESC.to_string(),
                block_size: FR_BLOCK_SIZE.to_string(),
            }
        } else {
            Messages {
                title: EN_TITLE.to_string(),
                author: EN_AUTHOR.to_string(),
                website: EN_WEBSITE.to_string(),
                usage: EN_USAGE.to_string(),
                memory_error: EN_MEMORY_ERROR.to_string(),
                memory_unit_error: EN_MEMORY_UNIT_ERROR.to_string(),
                swap_warning: EN_SWAP_WARNING.to_string(),
                total_memory: EN_TOTAL_MEMORY.to_string(),
                available_memory: EN_AVAILABLE_MEMORY.to_string(),
                filling_memory: EN_FILLING_MEMORY.to_string(),
                verifying_memory: EN_VERIFYING_MEMORY.to_string(),
                filling_complete: EN_FILLING_COMPLETE.to_string(),
                verification_complete: EN_VERIFICATION_COMPLETE.to_string(),
                test_success: EN_TEST_SUCCESS.to_string(),
                test_failed: EN_TEST_FAILED.to_string(),
                error_at_address: EN_ERROR_AT_ADDRESS.to_string(),
                summary: EN_SUMMARY.to_string(),
                loop_count: EN_LOOP_COUNT.to_string(),
                total_errors: EN_TOTAL_ERRORS.to_string(),
                total_tested: EN_TOTAL_TESTED.to_string(),
                total_time: EN_TOTAL_TIME.to_string(),
                interrupted: EN_INTERRUPTED.to_string(),
                error_map: EN_ERROR_MAP.to_string(),
                press_esc: EN_PRESS_ESC.to_string(),
                block_size: EN_BLOCK_SIZE.to_string(),
            }
        }
    }
}

fn detect_language() -> String {
    match env::var("LANG") {
        Ok(lang) => {
            if lang.contains("fr") {
                "fr".to_string()
            } else {
                "en".to_string()
            }
        }
        Err(_) => "en".to_string(),
    }
}

// Platform-specific memory detection with OS-specific percentages

#[cfg(target_os = "linux")]
fn get_available_memory() -> (u64, u64) {
    use std::fs;
    
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        let mut total_memory = 0;
        let mut available_memory = 0;
        
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    total_memory = value.parse::<u64>().unwrap_or(0) * 1024;
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    available_memory = value.parse::<u64>().unwrap_or(0) * 1024;
                }
            }
        }
        
        // Linux: use 90% of available memory
        let adjusted_available = (available_memory as f64 * 0.90) as u64;
        (total_memory, adjusted_available)
    } else {
        (0, 0)
    }
}

#[cfg(target_os = "freebsd")]
fn get_available_memory() -> (u64, u64) {
    use std::process::Command;
    use std::str;
    
    // Get total memory using sysctl hw.physmem
    let total_memory = if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.physmem").output() {
        if let Ok(value) = str::from_utf8(&output.stdout) {
            value.trim().parse::<u64>().unwrap_or(0)
        } else { 0 }
    } else { 0 };
    
    // Get available memory using sysctl vm.stats.vm.v_free_count
    let free_pages = if let Ok(output) = Command::new("sysctl").arg("-n").arg("vm.stats.vm.v_free_count").output() {
        if let Ok(value) = str::from_utf8(&output.stdout) {
            value.trim().parse::<u64>().unwrap_or(0)
        } else { 0 }
    } else { 0 };
    
    // Get page size
    let page_size = if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.pagesize").output() {
        if let Ok(value) = str::from_utf8(&output.stdout) {
            value.trim().parse::<u64>().unwrap_or(4096)
        } else { 4096 }
    } else { 4096 };
    
    let available_memory = free_pages * page_size;
    // FreeBSD: use 80% of available memory
    let adjusted_available = (available_memory as f64 * 0.80) as u64;
    
    (total_memory, adjusted_available)
}

#[cfg(target_os = "openbsd")]
fn get_available_memory() -> (u64, u64) {
    use std::process::Command;
    use std::str;
    
    // Get total memory using sysctl hw.physmem
    let total_memory = if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.physmem").output() {
        if let Ok(value) = str::from_utf8(&output.stdout) {
            value.trim().parse::<u64>().unwrap_or(0)
        } else { 0 }
    } else { 0 };
    
    // Get available memory using sysctl hw.usermem (approximation for OpenBSD)
    let available_memory = if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.usermem").output() {
        if let Ok(value) = str::from_utf8(&output.stdout) {
            value.trim().parse::<u64>().unwrap_or(total_memory)
        } else { total_memory }
    } else { total_memory };
    
    // OpenBSD: use 80% of available memory
    let adjusted_available = (available_memory as f64 * 0.80) as u64;
    (total_memory, adjusted_available)
}

#[cfg(target_os = "macos")]
fn get_available_memory() -> (u64, u64) {
    use std::process::Command;
    use std::str;
    
    // Get total memory using sysctl hw.memsize
    let total_memory = if let Ok(output) = Command::new("sysctl").arg("-n").arg("hw.memsize").output() {
        if let Ok(value) = str::from_utf8(&output.stdout) {
            value.trim().parse::<u64>().unwrap_or(0)
        } else { 0 }
    } else { 0 };
    
    // Get memory pressure info using vm_stat
    let available_memory = if let Ok(output) = Command::new("vm_stat").output() {
        let output_str = str::from_utf8(&output.stdout).unwrap_or("");
        let mut free_pages = 0;
        let mut inactive_pages = 0;
        let mut speculative_pages = 0;
        let page_size = 4096; // macOS typically uses 4KB pages
        
        for line in output_str.lines() {
            if line.contains("Pages free:") {
                if let Some(value) = line.split(':').nth(1) {
                    free_pages = value.trim().trim_end_matches('.').parse::<u64>().unwrap_or(0);
                }
            } else if line.contains("Pages inactive:") {
                if let Some(value) = line.split(':').nth(1) {
                    inactive_pages = value.trim().trim_end_matches('.').parse::<u64>().unwrap_or(0);
                }
            } else if line.contains("Pages speculative:") {
                if let Some(value) = line.split(':').nth(1) {
                    speculative_pages = value.trim().trim_end_matches('.').parse::<u64>().unwrap_or(0);
                }
            }
        }
        
        (free_pages + inactive_pages + speculative_pages) * page_size
    } else { total_memory / 2 }; // Fallback to half of total memory
    
    // macOS: use 90% of available memory (similar to Linux)
    let adjusted_available = (available_memory as f64 * 0.90) as u64;
    (total_memory, adjusted_available)
}

#[cfg(target_os = "windows")]
fn get_available_memory() -> (u64, u64) {
    use winapi::um::sysinfoapi::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    use std::mem;
    
    unsafe {
        let mut memory_status: MEMORYSTATUSEX = mem::zeroed();
        memory_status.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
        
        if GlobalMemoryStatusEx(&mut memory_status) != 0 {
            let total_memory = memory_status.ullTotalPhys;
            let available_memory = memory_status.ullAvailPhys;
            // Windows: use 90% of available memory
            let adjusted_available = (available_memory as f64 * 0.90) as u64;
            (total_memory, adjusted_available)
        } else {
            (0, 0)
        }
    }
}

// Fallback for other Unix-like systems
#[cfg(not(any(target_os = "linux", target_os = "freebsd", 
              target_os = "openbsd", target_os = "macos", 
              target_os = "windows")))]
fn get_available_memory() -> (u64, u64) {
    (0, 0)
}

fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec >= 1024.0 * 1024.0 * 1024.0 {
        format!("{:.2} GB/s", bytes_per_sec / (1024.0 * 1024.0 * 1024.0))
    } else if bytes_per_sec >= 1024.0 * 1024.0 {
        format!("{:.2} MB/s", bytes_per_sec / (1024.0 * 1024.0))
    } else if bytes_per_sec >= 1024.0 {
        format!("{:.2} KB/s", bytes_per_sec / 1024.0)
    } else {
        format!("{:.0} B/s", bytes_per_sec)
    }
}

#[cfg(not(target_os = "windows"))]
fn print_colored_text(color: &str, bold: &str, text: &str, reset: &str) {
    print!("{}{}{}{}", color, bold, text, reset);
}

#[cfg(target_os = "windows")]
fn print_colored_text(_color: &str, _bold: &str, text: &str, _reset: &str) {
    // On Windows, we ignore the ANSI codes and just print the text
    // Colors are handled by the Windows console API in the calling functions
    print!("{}", text);
}

#[cfg(not(target_os = "windows"))]
fn print_colored_line(color: &str, bold: &str, text: &str, reset: &str) {
    println!("{}{}{}{}", color, bold, text, reset);
}

#[cfg(target_os = "windows")]
fn print_colored_line(_color: &str, _bold: &str, text: &str, _reset: &str) {
    println!("{}", text);
}

#[cfg(not(target_os = "windows"))]
fn print_progress(progress: f32, elapsed: f64, speed: f64, phase: &str) {
    let remaining = if progress > 0.0 {
        (elapsed / progress as f64) * (100.0 - progress as f64)
    } else {
        0.0
    };
    
    let speed_str = format_speed(speed);
    
    print!("\r[");
    print!("{}{}", BRIGHT_CYAN, BOLD);
    print!("{:6.2}%", progress);
    print!("{}", RESET);
    print!("] ");
    print!("{}{}", BRIGHT_MAGENTA, BOLD);
    print!("{}", phase);
    print!("{}", RESET);
    print!(" - Elapsed: ");
    print!("{}{}", BRIGHT_YELLOW, BOLD);
    print!("{:6.0}", elapsed);
    print!("{}", RESET);
    print!("s - Remaining: ");
    print!("{}{}", BRIGHT_YELLOW, BOLD);
    print!("{:6.0}", remaining);
    print!("{}", RESET);
    print!("s - Speed: ");
    print!("{}{}", BRIGHT_GREEN, BOLD);
    print!("{}", speed_str);
    print!("{}   ", RESET);
    
    io::stdout().flush().unwrap();
}

#[cfg(target_os = "windows")]
fn print_progress(progress: f32, elapsed: f64, speed: f64, phase: &str) {
    use windows_console::*;
    
    let console = match ConsoleColor::new() {
        Ok(c) => c,
        Err(_) => {
            // Fallback if console color initialization fails
            print!("\r[{:6.2}%] {} - Elapsed: {:6.0}s - Remaining: {:6.0}s - Speed: {}   ", 
                progress, phase, elapsed, 
                if progress > 0.0 { (elapsed / progress as f64) * (100.0 - progress as f64) } else { 0.0 },
                format_speed(speed));
            io::stdout().flush().unwrap();
            return;
        }
    };
    
    let remaining = if progress > 0.0 {
        (elapsed / progress as f64) * (100.0 - progress as f64)
    } else {
        0.0
    };
    
    let speed_str = format_speed(speed);
    
    // Clear the line and print with Windows colors
    print!("\r");
    
    // Print percentage in cyan
    console.set_color(CYAN);
    print!("[{:6.2}%", progress);
    console.set_color(RESET);
    print!("] ");
    
    // Print phase in magenta
    console.set_color(MAGENTA);
    print!("{}", phase);
    console.set_color(RESET);
    print!(" - Elapsed: ");
    
    // Print elapsed time in yellow
    console.set_color(YELLOW);
    print!("{:6.0}", elapsed);
    console.set_color(RESET);
    print!("s - Remaining: ");
    
    // Print remaining time in yellow
    console.set_color(YELLOW);
    print!("{:6.0}", remaining);
    console.set_color(RESET);
    print!("s - Speed: ");
    
    // Print speed in green
    console.set_color(GREEN);
    print!("{}", speed_str);
    console.set_color(RESET);
    print!("   ");
    
    io::stdout().flush().unwrap();
}

fn show_error_map(mem_errors: &[bool], total_size: usize, msgs: &Messages) {
    const MAP_WIDTH: usize = 70;
    const MAP_HEIGHT: usize = 20;
    
    let errors_count = mem_errors.iter().filter(|&&x| x).count();
    
    if errors_count == 0 {
        #[cfg(not(target_os = "windows"))]
        println!("{}{}No errors to display.{}", BRIGHT_GREEN, BOLD, RESET);
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(GREEN);
                println!("No errors to display.");
                console.set_color(RESET);
            } else {
                println!("No errors to display.");
            }
        }
        return;
    }
    
    let blocks_per_char = (total_size / (MAP_WIDTH * MAP_HEIGHT)).max(1);
    let block_size = blocks_per_char;
    
    #[cfg(not(target_os = "windows"))]
    {
        println!("\n{}{}{} ({} {}){}", 
            BRIGHT_YELLOW, BOLD, msgs.error_map, errors_count, msgs.total_errors, RESET);
        println!("{}{}{}{} bytes{}", 
            BRIGHT_YELLOW, BOLD, msgs.block_size, block_size, RESET);
        println!("{}{}+{}+{}", 
            BRIGHT_YELLOW, BOLD, "-".repeat(MAP_WIDTH), RESET);
    }
    
    #[cfg(target_os = "windows")]
    {
        use windows_console::*;
        if let Ok(console) = ConsoleColor::new() {
            console.set_color(YELLOW);
            println!("\n{} ({} {})", msgs.error_map, errors_count, msgs.total_errors);
            println!("{}{} bytes", msgs.block_size, block_size);
            println!("+{}+", "-".repeat(MAP_WIDTH));
            console.set_color(RESET);
        } else {
            println!("\n{} ({} {})", msgs.error_map, errors_count, msgs.total_errors);
            println!("{}{} bytes", msgs.block_size, block_size);
            println!("+{}+", "-".repeat(MAP_WIDTH));
        }
    }
    
    for y in 0..MAP_HEIGHT {
        #[cfg(not(target_os = "windows"))]
        print!("{}{}|{}", BRIGHT_YELLOW, BOLD, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(YELLOW);
                print!("|");
                console.set_color(RESET);
            } else {
                print!("|");
            }
        }
        
        for x in 0..MAP_WIDTH {
            let mut has_error = false;
            let start_block = (y * MAP_WIDTH + x) * blocks_per_char;
            let end_block = (start_block + blocks_per_char).min(total_size);
            
            for i in start_block..end_block {
                if i < total_size && mem_errors[i] {
                    has_error = true;
                    break;
                }
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                if has_error {
                    print!("{}{}#{}", BRIGHT_RED, BOLD, RESET);
                } else {
                    print!("{}{}.{}", BRIGHT_GREEN, BOLD, RESET);
                }
            }
            
            #[cfg(target_os = "windows")]
            {
                use windows_console::*;
                if let Ok(console) = ConsoleColor::new() {
                    if has_error {
                        console.set_color(RED);
                        print!("#");
                    } else {
                        console.set_color(GREEN);
                        print!(".");
                    }
                    console.set_color(RESET);
                } else {
                    if has_error {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        println!("{}{}|{}", BRIGHT_YELLOW, BOLD, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(YELLOW);
                println!("|");
                console.set_color(RESET);
            } else {
                println!("|");
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        println!("{}{}+{}+{}", 
            BRIGHT_YELLOW, BOLD, "-".repeat(MAP_WIDTH), RESET);
        println!("{}{}{}{}", BRIGHT_YELLOW, BOLD, msgs.press_esc, RESET);
    }
    
    #[cfg(target_os = "windows")]
    {
        use windows_console::*;
        if let Ok(console) = ConsoleColor::new() {
            console.set_color(YELLOW);
            println!("+{}+", "-".repeat(MAP_WIDTH));
            println!("{}", msgs.press_esc);
            console.set_color(RESET);
        } else {
            println!("+{}+", "-".repeat(MAP_WIDTH));
            println!("{}", msgs.press_esc);
        }
    }
    
    // Simple keyboard input reading
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    loop {
        if let Ok(_) = stdin.read_exact(&mut buffer) {
            if buffer[0] == 27 || buffer[0] == b'q' || buffer[0] == b'Q' {
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn check_interruption(interrupted: &Arc<AtomicBool>) -> bool {
    interrupted.load(Ordering::Relaxed)
}

fn test_ram(
    size: usize,
    pattern: u8,
    total_errors: &mut u64,
    interrupted: &Arc<AtomicBool>,
    total_tested: &mut u64,
    test_duration: &mut f64,
    msgs: &Messages,
) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    let mut mem_errors = vec![false; size];
    let mut errors = 0;
    
    // Allocate memory
    let mut memory = vec![0u8; size];
    
    // Filling phase
    #[cfg(not(target_os = "windows"))]
    println!("\n{}{}{}{}", BRIGHT_BLUE, BOLD, msgs.filling_memory, RESET);
    
    #[cfg(target_os = "windows")]
    {
        use windows_console::*;
        if let Ok(console) = ConsoleColor::new() {
            console.set_color(BLUE);
            println!("\n{}", msgs.filling_memory);
            console.set_color(RESET);
        } else {
            println!("\n{}", msgs.filling_memory);
        }
    }
    
    let phase_start = Instant::now();
    let mut last_update = Instant::now();
    let mut last_bytes_processed = 0;
    
    for i in 0..size {
        memory[i] = ((i as u64 ^ pattern as u64) % 256) as u8;
        
        if i % update_interval == 0 && i > 0 {
            let now = Instant::now();
            let elapsed = phase_start.elapsed().as_secs_f64();
            let progress = (i as f32 / size as f32) * 100.0;
            
            // Calculate speed
            let time_since_last = now.duration_since(last_update).as_secs_f64();
            let bytes_since_last = (i - last_bytes_processed) as f64;
            let instant_speed = if time_since_last > 0.0 {
                bytes_since_last / time_since_last
            } else {
                0.0
            };
            
            print_progress(progress, elapsed, instant_speed, &msgs.filling_memory);
            
            last_update = now;
            last_bytes_processed = i;
            
            if check_interruption(interrupted) {
                #[cfg(not(target_os = "windows"))]
                println!("\n{}{}{}{}", BRIGHT_RED, BOLD, msgs.interrupted, RESET);
                
                #[cfg(target_os = "windows")]
                {
                    use windows_console::*;
                    if let Ok(console) = ConsoleColor::new() {
                        console.set_color(RED);
                        println!("\n{}", msgs.interrupted);
                        console.set_color(RESET);
                    } else {
                        println!("\n{}", msgs.interrupted);
                    }
                }
                return true;
            }
        }
    }
    
    if !check_interruption(interrupted) {
        *test_duration += phase_start.elapsed().as_secs_f64();
        let total_elapsed = phase_start.elapsed().as_secs_f64();
        let avg_speed = size as f64 / total_elapsed;
        print_progress(100.0, total_elapsed, avg_speed, &msgs.filling_memory);
        
        #[cfg(not(target_os = "windows"))]
        println!("\n{}{}{}{:.2}s (avg: {}){}", 
            BRIGHT_GREEN, BOLD, msgs.filling_complete, total_elapsed, format_speed(avg_speed), RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(GREEN);
                println!("\n{}{:.2}s (avg: {})", msgs.filling_complete, total_elapsed, format_speed(avg_speed));
                console.set_color(RESET);
            } else {
                println!("\n{}{:.2}s (avg: {})", msgs.filling_complete, total_elapsed, format_speed(avg_speed));
            }
        }
    }
    
    // Verification phase
    if !check_interruption(interrupted) {
        #[cfg(not(target_os = "windows"))]
        println!("\n{}{}{}{}", BRIGHT_BLUE, BOLD, msgs.verifying_memory, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(BLUE);
                println!("\n{}", msgs.verifying_memory);
                console.set_color(RESET);
            } else {
                println!("\n{}", msgs.verifying_memory);
            }
        }
        
        let phase_start = Instant::now();
        let mut last_update = Instant::now();
        let mut last_bytes_processed = 0;
        
        for i in 0..size {
            let expected_value = ((i as u64 ^ pattern as u64) % 256) as u8;
            if memory[i] != expected_value {
                errors += 1;
                mem_errors[i] = true;
                
                #[cfg(not(target_os = "windows"))]
                println!("\n{}{}{}0x{:016x} (got:{} vs exp:{}){}", 
                    BRIGHT_RED, BOLD, msgs.error_at_address, 
                    &memory[i] as *const u8 as usize,
                    memory[i], 
                    expected_value,
                    RESET);
                
                #[cfg(target_os = "windows")]
                {
                    use windows_console::*;
                    if let Ok(console) = ConsoleColor::new() {
                        console.set_color(RED);
                        println!("\n{}0x{:016x} (got:{} vs exp:{})", 
                            msgs.error_at_address, 
                            &memory[i] as *const u8 as usize,
                            memory[i], 
                            expected_value);
                        console.set_color(RESET);
                    } else {
                        println!("\n{}0x{:016x} (got:{} vs exp:{})", 
                            msgs.error_at_address, 
                            &memory[i] as *const u8 as usize,
                            memory[i], 
                            expected_value);
                    }
                }
            }
            
            if i % update_interval == 0 && i > 0 {
                let now = Instant::now();
                let elapsed = phase_start.elapsed().as_secs_f64();
                let progress = (i as f32 / size as f32) * 100.0;
                
                // Calculate speed
                let time_since_last = now.duration_since(last_update).as_secs_f64();
                let bytes_since_last = (i - last_bytes_processed) as f64;
                let instant_speed = if time_since_last > 0.0 {
                    bytes_since_last / time_since_last
                } else {
                    0.0
                };
                
                print_progress(progress, elapsed, instant_speed, &msgs.verifying_memory);
                
                last_update = now;
                last_bytes_processed = i;
                
                if check_interruption(interrupted) {
                    #[cfg(not(target_os = "windows"))]
                    println!("\n{}{}{}{}", BRIGHT_RED, BOLD, msgs.interrupted, RESET);
                    
                    #[cfg(target_os = "windows")]
                    {
                        use windows_console::*;
                        if let Ok(console) = ConsoleColor::new() {
                            console.set_color(RED);
                            println!("\n{}", msgs.interrupted);
                            console.set_color(RESET);
                        } else {
                            println!("\n{}", msgs.interrupted);
                        }
                    }
                    return true;
                }
            }
        }
        
        if !check_interruption(interrupted) {
            *test_duration += phase_start.elapsed().as_secs_f64();
            let total_elapsed = phase_start.elapsed().as_secs_f64();
            let avg_speed = size as f64 / total_elapsed;
            print_progress(100.0, total_elapsed, avg_speed, &msgs.verifying_memory);
            
            #[cfg(not(target_os = "windows"))]
            println!("\n{}{}{}{:.2}s (avg: {}){}", 
                BRIGHT_GREEN, BOLD, msgs.verification_complete, total_elapsed, format_speed(avg_speed), RESET);
            
            #[cfg(target_os = "windows")]
            {
                use windows_console::*;
                if let Ok(console) = ConsoleColor::new() {
                    console.set_color(GREEN);
                    println!("\n{}{:.2}s (avg: {})", msgs.verification_complete, total_elapsed, format_speed(avg_speed));
                    console.set_color(RESET);
                } else {
                    println!("\n{}{:.2}s (avg: {})", msgs.verification_complete, total_elapsed, format_speed(avg_speed));
                }
            }
        }
    }
    
    // Results
    if errors == 0 {
        #[cfg(not(target_os = "windows"))]
        println!("\n{}{}{}{}", BRIGHT_GREEN, BOLD, msgs.test_success, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(GREEN);
                println!("\n{}", msgs.test_success);
                console.set_color(RESET);
            } else {
                println!("\n{}", msgs.test_success);
            }
        }
    } else {
        #[cfg(not(target_os = "windows"))]
        println!("\n{}{}{}{} errors.{}", BRIGHT_RED, BOLD, msgs.test_failed, errors, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(RED);
                println!("\n{}{} errors.", msgs.test_failed, errors);
                console.set_color(RESET);
            } else {
                println!("\n{}{} errors.", msgs.test_failed, errors);
            }
        }
        show_error_map(&mem_errors, size, msgs);
    }
    
    *total_errors += errors;
    *total_tested += size as u64;
    
    check_interruption(interrupted)
}

fn main() {
    // Initialize Windows console if needed
    #[cfg(target_os = "windows")]
    let _console = windows_console::ConsoleColor::new();
    
    let lang = detect_language();
    let msgs = Messages::new(&lang);
    
    let args: Vec<String> = env::args().collect();
    
    #[cfg(not(target_os = "windows"))]
    {
        println!("{}{}{}", BRIGHT_WHITE, BOLD, msgs.title);
        println!("{}{}", BRIGHT_WHITE, msgs.author);
        println!("{}{}{}", BRIGHT_WHITE, msgs.website, RESET);
    }
    
    #[cfg(target_os = "windows")]
    {
        use windows_console::*;
        if let Ok(console) = ConsoleColor::new() {
            console.set_color(WHITE);
            println!("{}", msgs.title);
            println!("{}", msgs.author);
            println!("{}", msgs.website);
            console.set_color(RESET);
        } else {
            println!("{}", msgs.title);
            println!("{}", msgs.author);
            println!("{}", msgs.website);
        }
    }
    println!();
    
    if args.len() < 2 {
        #[cfg(not(target_os = "windows"))]
        println!("{}{}{}{}", BRIGHT_YELLOW, BOLD, msgs.usage, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(YELLOW);
                println!("{}", msgs.usage);
                console.set_color(RESET);
            } else {
                println!("{}", msgs.usage);
            }
        }
        return;
    }
    
    let arg = &args[1];
    let loop_mode = args.len() > 2 && args[2] == "--loop";
    
    let (total_memory, available_mem) = get_available_memory();
    
    let size = if arg.to_uppercase() == "MAX" {
        available_mem as usize
    } else {
        let unit = arg.chars().last().unwrap_or(' ').to_ascii_uppercase();
        let value_str = if arg.len() > 1 { &arg[..arg.len()-1] } else { arg };
        let value = value_str.parse::<u64>().unwrap_or(0);
        
        match unit {
            'M' => (value * 1024 * 1024) as usize,
            'G' => (value * 1024 * 1024 * 1024) as usize,
            _ => {
                #[cfg(not(target_os = "windows"))]
                println!("{}{}{}{}", BRIGHT_RED, BOLD, msgs.memory_unit_error, RESET);
                
                #[cfg(target_os = "windows")]
                {
                    use windows_console::*;
                    if let Ok(console) = ConsoleColor::new() {
                        console.set_color(RED);
                        println!("{}", msgs.memory_unit_error);
                        console.set_color(RESET);
                    } else {
                        println!("{}", msgs.memory_unit_error);
                    }
                }
                return;
            }
        }
    };
    
    if arg.to_uppercase() != "MAX" && size > available_mem as usize {
        #[cfg(not(target_os = "windows"))]
        println!("{}{}{}{}\n", BRIGHT_RED, BOLD, msgs.swap_warning, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(RED);
                println!("{}\n", msgs.swap_warning);
                console.set_color(RESET);
            } else {
                println!("{}\n", msgs.swap_warning);
            }
        }
    }
    
    if total_memory > 0 {
        #[cfg(not(target_os = "windows"))]
        {
            println!("{}{}{:.1} MB.{}", 
                BRIGHT_WHITE, msgs.total_memory, total_memory as f64 / (1024.0 * 1024.0), RESET);
            println!("{}{}{} bytes ({} MB){}", 
                BRIGHT_WHITE, msgs.available_memory, 
                available_mem, 
                available_mem / (1024 * 1024),
                RESET);
        }
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(WHITE);
                println!("{}{:.1} MB.", msgs.total_memory, total_memory as f64 / (1024.0 * 1024.0));
                println!("{}{} bytes ({} MB)", 
                    msgs.available_memory, 
                    available_mem, 
                    available_mem / (1024 * 1024));
                console.set_color(RESET);
            } else {
                println!("{}{:.1} MB.", msgs.total_memory, total_memory as f64 / (1024.0 * 1024.0));
                println!("{}{} bytes ({} MB)", 
                    msgs.available_memory, 
                    available_mem, 
                    available_mem / (1024 * 1024));
            }
        }
    } else {
        println!("Could not determine system memory. Assuming {} bytes available.", available_mem);
    }
    println!();
    
    let interrupted = Arc::new(AtomicBool::new(false));
    
    let mut rng = rand::thread_rng();
    let mut loop_count = 0;
    let mut total_errors = 0;
    let mut total_tested = 0;
    let mut total_time = 0.0;
    
    if loop_mode {
        loop {
            let pattern = rng.gen_range(0..=255u8);
            loop_count += 1;
            
            #[cfg(not(target_os = "windows"))]
            println!("{}{}Loop #{} - Pattern: {}{}", 
                BRIGHT_CYAN, BOLD, loop_count, pattern, RESET);
            
            #[cfg(target_os = "windows")]
            {
                use windows_console::*;
                if let Ok(console) = ConsoleColor::new() {
                    console.set_color(CYAN);
                    println!("Loop #{} - Pattern: {}", loop_count, pattern);
                    console.set_color(RESET);
                } else {
                    println!("Loop #{} - Pattern: {}", loop_count, pattern);
                }
            }
            
            let current_size = if arg.to_uppercase() == "MAX" {
                let (_, avail) = get_available_memory();
                
                #[cfg(not(target_os = "windows"))]
                println!("{}{}{} bytes ({} MB){}", 
                    BRIGHT_WHITE, msgs.available_memory, 
                    avail, 
                    avail / (1024 * 1024),
                    RESET);
                
                #[cfg(target_os = "windows")]
                {
                    use windows_console::*;
                    if let Ok(console) = ConsoleColor::new() {
                        console.set_color(WHITE);
                        println!("{}{} bytes ({} MB)", 
                            msgs.available_memory, 
                            avail, 
                            avail / (1024 * 1024));
                        console.set_color(RESET);
                    } else {
                        println!("{}{} bytes ({} MB)", 
                            msgs.available_memory, 
                            avail, 
                            avail / (1024 * 1024));
                    }
                }
                avail as usize
            } else {
                size
            };
            
            let was_interrupted = test_ram(
                current_size, 
                pattern, 
                &mut total_errors, 
                &interrupted, 
                &mut total_tested, 
                &mut total_time,
                &msgs
            );
            
            if was_interrupted || check_interruption(&interrupted) {
                break;
            }
            
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        test_ram(
            size, 
            255, 
            &mut total_errors, 
            &interrupted, 
            &mut total_tested, 
            &mut total_time,
            &msgs
        );
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        println!("\n{}{}{}{}", BRIGHT_YELLOW, BOLD, msgs.summary, RESET);
        if loop_count > 0 {
            println!("{}{}{}", BRIGHT_WHITE, msgs.loop_count, loop_count);
        }
        println!("{}{}{}", BRIGHT_WHITE, msgs.total_errors, total_errors);
        println!("{}{}{} bytes ({} MB)", 
            BRIGHT_WHITE, msgs.total_tested, 
            total_tested, 
            total_tested / (1024 * 1024));
    }
    
    #[cfg(target_os = "windows")]
    {
        use windows_console::*;
        if let Ok(console) = ConsoleColor::new() {
            console.set_color(YELLOW);
            println!("\n{}", msgs.summary);
            console.set_color(WHITE);
            if loop_count > 0 {
                println!("{}{}", msgs.loop_count, loop_count);
            }
            println!("{}{}", msgs.total_errors, total_errors);
            println!("{}{} bytes ({} MB)", 
                msgs.total_tested, 
                total_tested, 
                total_tested / (1024 * 1024));
            console.set_color(RESET);
        } else {
            println!("\n{}", msgs.summary);
            if loop_count > 0 {
                println!("{}{}", msgs.loop_count, loop_count);
            }
            println!("{}{}", msgs.total_errors, total_errors);
            println!("{}{} bytes ({} MB)", 
                msgs.total_tested, 
                total_tested, 
                total_tested / (1024 * 1024));
        }
    }
    
    // Calculate and display average speed for the entire test
    if total_time > 0.0 {
        let avg_speed = total_tested as f64 / total_time;
        
        #[cfg(not(target_os = "windows"))]
        println!("{}{}{:.2}s (avg: {})", 
            BRIGHT_WHITE, msgs.total_time, total_time, format_speed(avg_speed));
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(WHITE);
                println!("{}{:.2}s (avg: {})", msgs.total_time, total_time, format_speed(avg_speed));
                console.set_color(RESET);
            } else {
                println!("{}{:.2}s (avg: {})", msgs.total_time, total_time, format_speed(avg_speed));
            }
        }
    } else {
        #[cfg(not(target_os = "windows"))]
        println!("{}{}{:.2}s{}", BRIGHT_WHITE, msgs.total_time, total_time, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(WHITE);
                println!("{}{:.2}s", msgs.total_time, total_time);
                console.set_color(RESET);
            } else {
                println!("{}{:.2}s", msgs.total_time, total_time);
            }
        }
    }
    println!();
    
    if check_interruption(&interrupted) {
        #[cfg(not(target_os = "windows"))]
        println!("{}{}{}{}", BRIGHT_RED, BOLD, msgs.interrupted, RESET);
        
        #[cfg(target_os = "windows")]
        {
            use windows_console::*;
            if let Ok(console) = ConsoleColor::new() {
                console.set_color(RED);
                println!("{}", msgs.interrupted);
                console.set_color(RESET);
            } else {
                println!("{}", msgs.interrupted);
            }
        }
    }
}

