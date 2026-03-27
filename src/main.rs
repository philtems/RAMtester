use std::env;
use std::io::{self, Write, Read};
use std::time::{Instant, Duration};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use rand::Rng;

// Constantes pour les messages en français
const FR_TITLE: &str = "RAMtester v3.0";
const FR_AUTHOR: &str = "2025-2026, Philippe TEMESI";
const FR_WEBSITE: &str = "https://www.tems.be";
const FR_USAGE: &str = "Utilisation : ramtester <taille> (ex: 512M, 2G ou MAX) [--loop] [--test <N>] [--ultra]";
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
const FR_TEST_LIST: &str = "Tests disponibles :";
const FR_TEST_RUNNING: &str = "Test en cours : ";
const FR_TEST_SELECT: &str = "Test sélectionné : ";
const FR_ULTRA_MODE: &str = "Mode ULTRA : tous les tests (1-16)";
const FR_NORMAL_MODE: &str = "Mode NORMAL : tests 1-10 et 12-16";
const FR_TEST_COMPLETE: &str = "Test terminé ";
const FR_ALL_TESTS_COMPLETE: &str = "Tous les tests terminés";

// Messages en anglais
const EN_TITLE: &str = "RAMtester v3.0";
const EN_AUTHOR: &str = "2025-2026, Philippe TEMESI";
const EN_WEBSITE: &str = "https://www.tems.be";
const EN_USAGE: &str = "Usage: ramtester <size> (ex: 512M, 2G or MAX) [--loop] [--test <N>] [--ultra]";
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
const EN_TEST_LIST: &str = "Available tests:";
const EN_TEST_RUNNING: &str = "Running test: ";
const EN_TEST_SELECT: &str = "Selected test: ";
const EN_ULTRA_MODE: &str = "ULTRA mode: all tests (1-16)";
const EN_NORMAL_MODE: &str = "NORMAL mode: tests 1-10 and 12-16";
const EN_TEST_COMPLETE: &str = "Test completed ";
const EN_ALL_TESTS_COMPLETE: &str = "All tests completed";

// Color constants for ANSI terminals
#[cfg(not(target_os = "windows"))]
const BRIGHT_WHITE: &str = "\x1b[97m";
#[cfg(not(target_os = "windows"))]
const BRIGHT_CYAN: &str = "\x1b[96m";
#[cfg(not(target_os = "windows"))]
const BRIGHT_MAGENTA: &str = "\x1b[95m";
#[cfg(not(target_os = "windows"))]
const BRIGHT_YELLOW: &str = "\x1b[93m";
#[cfg(not(target_os = "windows"))]
const BRIGHT_GREEN: &str = "\x1b[92m";
#[cfg(not(target_os = "windows"))]
const BRIGHT_RED: &str = "\x1b[91m";
#[cfg(not(target_os = "windows"))]
const BRIGHT_BLUE: &str = "\x1b[94m";
#[cfg(not(target_os = "windows"))]
const RESET: &str = "\x1b[0m";
#[cfg(not(target_os = "windows"))]
const BOLD: &str = "\x1b[1m";

// Color constants for Windows (empty strings)
#[cfg(target_os = "windows")]
const BRIGHT_WHITE: &str = "";
#[cfg(target_os = "windows")]
const BRIGHT_CYAN: &str = "";
#[cfg(target_os = "windows")]
const BRIGHT_MAGENTA: &str = "";
#[cfg(target_os = "windows")]
const BRIGHT_YELLOW: &str = "";
#[cfg(target_os = "windows")]
const BRIGHT_GREEN: &str = "";
#[cfg(target_os = "windows")]
const BRIGHT_RED: &str = "";
#[cfg(target_os = "windows")]
const BRIGHT_BLUE: &str = "";
#[cfg(target_os = "windows")]
const RESET: &str = "";
#[cfg(target_os = "windows")]
const BOLD: &str = "";

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
    test_list: String,
    test_running: String,
    test_select: String,
    ultra_mode: String,
    normal_mode: String,
    test_complete: String,
    all_tests_complete: String,
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
                test_list: FR_TEST_LIST.to_string(),
                test_running: FR_TEST_RUNNING.to_string(),
                test_select: FR_TEST_SELECT.to_string(),
                ultra_mode: FR_ULTRA_MODE.to_string(),
                normal_mode: FR_NORMAL_MODE.to_string(),
                test_complete: FR_TEST_COMPLETE.to_string(),
                all_tests_complete: FR_ALL_TESTS_COMPLETE.to_string(),
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
                test_list: EN_TEST_LIST.to_string(),
                test_running: EN_TEST_RUNNING.to_string(),
                test_select: EN_TEST_SELECT.to_string(),
                ultra_mode: EN_ULTRA_MODE.to_string(),
                normal_mode: EN_NORMAL_MODE.to_string(),
                test_complete: EN_TEST_COMPLETE.to_string(),
                all_tests_complete: EN_ALL_TESTS_COMPLETE.to_string(),
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

// Platform-specific memory detection
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
        
        let adjusted_available = (available_memory as f64 * 0.90) as u64;
        (total_memory, adjusted_available)
    } else {
        (0, 0)
    }
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
            let adjusted_available = (available_memory as f64 * 0.90) as u64;
            (total_memory, adjusted_available)
        } else {
            (0, 0)
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn get_available_memory() -> (u64, u64) {
    (0, 0)
}

// Types for tests
type TestFn = fn(&mut [u8], &mut [u8], usize, &Arc<AtomicBool>) -> bool;

#[derive(Clone, Copy)]
struct Test {
    name: &'static str,
    func: TestFn,
}

const ALL_TESTS: [Test; 16] = [
    Test { name: "Standard Fill Test", func: test_standard },
    Test { name: "Random Value", func: test_random_value },
    Test { name: "Compare XOR", func: test_xor_comparison },
    Test { name: "Compare SUB", func: test_sub_comparison },
    Test { name: "Compare MUL", func: test_mul_comparison },
    Test { name: "Compare DIV", func: test_div_comparison },
    Test { name: "Compare OR", func: test_or_comparison },
    Test { name: "Compare AND", func: test_and_comparison },
    Test { name: "Sequential Increment", func: test_seqinc_comparison },
    Test { name: "Solid Bits", func: test_solidbits_comparison },
    Test { name: "Block Sequential", func: test_blockseq_comparison },
    Test { name: "Checkerboard", func: test_checkerboard_comparison },
    Test { name: "Bit Spread", func: test_bitspread_comparison },
    Test { name: "Bit Flip", func: test_bitflip_comparison },
    Test { name: "Walking Ones", func: test_walkbits1_comparison },
    Test { name: "Walking Zeroes", func: test_walkbits0_comparison },
];

// Default tests: all tests except #11 (Block Sequential) which is index 10
const DEFAULT_TESTS: &[Test] = &[
    ALL_TESTS[0],  // 1. Standard Fill Test
    ALL_TESTS[1],  // 2. Random Value
    ALL_TESTS[2],  // 3. Compare XOR
    ALL_TESTS[3],  // 4. Compare SUB
    ALL_TESTS[4],  // 5. Compare MUL
    ALL_TESTS[5],  // 6. Compare DIV
    ALL_TESTS[6],  // 7. Compare OR
    ALL_TESTS[7],  // 8. Compare AND
    ALL_TESTS[8],  // 9. Sequential Increment
    ALL_TESTS[9],  // 10. Solid Bits
    ALL_TESTS[11], // 12. Checkerboard (skip index 10 - Block Sequential)
    ALL_TESTS[12], // 13. Bit Spread
    ALL_TESTS[13], // 14. Bit Flip
    ALL_TESTS[14], // 15. Walking Ones
    ALL_TESTS[15], // 16. Walking Zeroes
];

// Utility functions for tests
fn compare_regions(bufa: &[u8], bufb: &[u8], size: usize, errors: &mut Vec<bool>, msgs: &Messages) -> bool {
    let mut has_error = false;
    for i in 0..size {
        if bufa[i] != bufb[i] {
            has_error = true;
            errors[i] = true;
            println!("\n{}{}{}0x{:016x} (got:{} vs exp:{}){}", 
                BRIGHT_RED, BOLD, msgs.error_at_address, 
                &bufa[i] as *const u8 as usize + i,
                bufa[i], 
                bufb[i],
                RESET);
        }
    }
    has_error
}

fn test_standard(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let pattern = 255;
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        bufa[i] = ((i as u64 ^ pattern as u64) % 256) as u8;
        bufb[i] = bufa[i];
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    
    false
}

fn test_random_value(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        let val = rng.gen::<u8>();
        bufa[i] = val;
        bufb[i] = val;
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_xor_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let q = rng.gen::<u8>();
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        bufa[i] ^= q;
        bufb[i] ^= q;
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_sub_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let q = rng.gen::<u8>();
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        bufa[i] = bufa[i].wrapping_sub(q);
        bufb[i] = bufb[i].wrapping_sub(q);
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_mul_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let q = rng.gen::<u8>();
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        bufa[i] = bufa[i].wrapping_mul(q);
        bufb[i] = bufb[i].wrapping_mul(q);
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_div_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let mut q = rng.gen::<u8>();
    if q == 0 {
        q = 1;
    }
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        bufa[i] = bufa[i] / q;
        bufb[i] = bufb[i] / q;
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_or_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let q = rng.gen::<u8>();
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        bufa[i] |= q;
        bufb[i] |= q;
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_and_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let q = rng.gen::<u8>();
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        bufa[i] &= q;
        bufb[i] &= q;
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_seqinc_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let mut rng = rand::thread_rng();
    let q = rng.gen::<u8>();
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for i in 0..size {
        let val = ((i as u64).wrapping_add(q as u64) % 256) as u8;
        bufa[i] = val;
        bufb[i] = val;
        
        if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
            return true;
        }
    }
    false
}

fn test_solidbits_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for j in 0..8 {
        let q = if (j % 2) == 0 { 0xFFu8 } else { 0x00u8 };
        for i in 0..size {
            let val = if (i % 2) == 0 { q } else { !q };
            bufa[i] = val;
            bufb[i] = val;
            
            if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
                return true;
            }
        }
    }
    false
}

fn test_blockseq_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for j in 0..256 {
        let val = j as u8;
        for i in 0..size {
            bufa[i] = val;
            bufb[i] = val;
            
            if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
                return true;
            }
        }
    }
    false
}

fn test_checkerboard_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for j in 0..8 {
        let q = if (j % 2) == 0 { 0x55u8 } else { 0xAAu8 };
        for i in 0..size {
            let val = if (i % 2) == 0 { q } else { !q };
            bufa[i] = val;
            bufb[i] = val;
            
            if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
                return true;
            }
        }
    }
    false
}

fn test_bitspread_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for j in 0..16 {
        for i in 0..size {
            let val = if j < 8 {
                if (i % 2) == 0 {
                    (1 << j) | (1 << (j + 2))
                } else {
                    !((1 << j) | (1 << (j + 2)))
                }
            } else {
                let idx = 15 - j;
                if (i % 2) == 0 {
                    (1 << idx) | (1 << (idx + 2))
                } else {
                    !((1 << idx) | (1 << (idx + 2)))
                }
            };
            bufa[i] = val as u8;
            bufb[i] = val as u8;
            
            if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
                return true;
            }
        }
    }
    false
}

fn test_bitflip_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for k in 0..8 {
        let mut q = 1 << k;
        for _j in 0..2 {
            q = !q;
            for i in 0..size {
                let val = if (i % 2) == 0 { q as u8 } else { !(q as u8) };
                bufa[i] = val;
                bufb[i] = val;
                
                if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
                    return true;
                }
            }
        }
    }
    false
}

fn test_walkbits0_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for j in 0..16 {
        for i in 0..size {
            let val = if j < 8 {
                1 << j
            } else {
                1 << (15 - j)
            };
            bufa[i] = val as u8;
            bufb[i] = val as u8;
            
            if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
                return true;
            }
        }
    }
    false
}

fn test_walkbits1_comparison(bufa: &mut [u8], bufb: &mut [u8], size: usize, interrupted: &Arc<AtomicBool>) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    
    for j in 0..16 {
        for i in 0..size {
            let val = if j < 8 {
                0xFF ^ (1 << j)
            } else {
                0xFF ^ (1 << (15 - j))
            };
            bufa[i] = val as u8;
            bufb[i] = val as u8;
            
            if i % update_interval == 0 && i > 0 && check_interruption(interrupted) {
                return true;
            }
        }
    }
    false
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

fn show_error_map(mem_errors: &[bool], total_size: usize, msgs: &Messages) {
    const MAP_WIDTH: usize = 70;
    const MAP_HEIGHT: usize = 20;
    
    let errors_count = mem_errors.iter().filter(|&&x| x).count();
    
    if errors_count == 0 {
        println!("{}{}No errors to display.{}", BRIGHT_GREEN, BOLD, RESET);
        return;
    }
    
    let blocks_per_char = (total_size / (MAP_WIDTH * MAP_HEIGHT)).max(1);
    let block_size = blocks_per_char;
    
    println!("\n{}{}{} ({} {}){}", 
        BRIGHT_YELLOW, BOLD, msgs.error_map, errors_count, msgs.total_errors, RESET);
    println!("{}{}{}{} bytes{}", 
        BRIGHT_YELLOW, BOLD, msgs.block_size, block_size, RESET);
    println!("{}{}+{}+{}", 
        BRIGHT_YELLOW, BOLD, "-".repeat(MAP_WIDTH), RESET);
    
    for y in 0..MAP_HEIGHT {
        print!("{}{}|{}", BRIGHT_YELLOW, BOLD, RESET);
        
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
            
            if has_error {
                print!("{}{}#{}", BRIGHT_RED, BOLD, RESET);
            } else {
                print!("{}{}.{}", BRIGHT_GREEN, BOLD, RESET);
            }
        }
        
        println!("{}{}|{}", BRIGHT_YELLOW, BOLD, RESET);
    }
    
    println!("{}{}+{}+{}", 
        BRIGHT_YELLOW, BOLD, "-".repeat(MAP_WIDTH), RESET);
    println!("{}{}{}{}", BRIGHT_YELLOW, BOLD, msgs.press_esc, RESET);
    
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

fn run_test(
    test: &Test,
    size: usize,
    total_errors: &mut u64,
    interrupted: &Arc<AtomicBool>,
    total_tested: &mut u64,
    test_duration: &mut f64,
    msgs: &Messages,
    test_index: Option<usize>,
    total_tests: Option<usize>,
) -> bool {
    let update_interval = (128 * 1024 * 1024).min(size / 100).max(1);
    let mut mem_errors = vec![false; size];
    let mut errors = 0;
    
    // Allocate a single buffer and split it into two halves
    let mut buffer = vec![0u8; size];
    let halflen = size / 2;
    let (first_half, second_half) = buffer.split_at_mut(halflen);
    
    // Display test name with progress if in all-tests mode
    if let (Some(idx), Some(total)) = (test_index, total_tests) {
        println!("\n{}{}{} [{}/{}] - {}{}", 
            BRIGHT_BLUE, BOLD, msgs.test_running, idx + 1, total, test.name, RESET);
    } else {
        println!("\n{}{}{}{}{}", BRIGHT_BLUE, BOLD, msgs.test_running, test.name, RESET);
    }
    
    let phase_start = Instant::now();
    
    // Run the test function on the two halves
    let was_interrupted = (test.func)(first_half, second_half, halflen, interrupted);
    
    if was_interrupted {
        return true;
    }
    
    // Verification phase - compare the two halves
    println!("{}{}{}", BRIGHT_BLUE, msgs.verifying_memory, RESET);
    
    let verify_start = Instant::now();
    let mut last_update_verify = Instant::now();
    let mut last_bytes_processed_verify = 0;
    
    for i in 0..halflen {
        if first_half[i] != second_half[i] {
            errors += 1;
            mem_errors[i] = true;
            
            println!("\n{}{}{}0x{:016x} (got:{} vs exp:{}){}", 
                BRIGHT_RED, BOLD, msgs.error_at_address, 
                &first_half[i] as *const u8 as usize + i,
                first_half[i], 
                second_half[i],
                RESET);
        }
        
        if i % update_interval == 0 && i > 0 {
            let now = Instant::now();
            let elapsed = verify_start.elapsed().as_secs_f64();
            let progress = (i as f32 / halflen as f32) * 100.0;
            
            let time_since_last = now.duration_since(last_update_verify).as_secs_f64();
            let bytes_since_last = (i - last_bytes_processed_verify) as f64;
            let instant_speed = if time_since_last > 0.0 {
                bytes_since_last / time_since_last
            } else {
                0.0
            };
            
            print_progress(progress, elapsed, instant_speed, &msgs.verifying_memory);
            
            last_update_verify = now;
            last_bytes_processed_verify = i;
            
            if check_interruption(interrupted) {
                println!("\n{}{}{}{}", BRIGHT_RED, BOLD, msgs.interrupted, RESET);
                return true;
            }
        }
    }
    
    if !check_interruption(interrupted) {
        *test_duration += phase_start.elapsed().as_secs_f64() + verify_start.elapsed().as_secs_f64();
        let total_elapsed = verify_start.elapsed().as_secs_f64();
        let avg_speed = halflen as f64 / total_elapsed;
        print_progress(100.0, total_elapsed, avg_speed, &msgs.verifying_memory);
        
        println!("\n{}{}{}{:.2}s (avg: {}){}", 
            BRIGHT_GREEN, BOLD, msgs.test_complete, total_elapsed, format_speed(avg_speed), RESET);
    }
    
    // Results
    if errors == 0 {
        println!("{}{}{}{}", BRIGHT_GREEN, BOLD, msgs.test_success, RESET);
    } else {
        println!("{}{}{}{} errors.{}", BRIGHT_RED, BOLD, msgs.test_failed, errors, RESET);
        
        // Only show error map if there are errors and we're not in all-tests mode with many tests
        if test_index.is_none() {
            show_error_map(&mem_errors, halflen, msgs);
        } else if errors > 0 {
            println!("{}{}Use --test {} to see detailed error map.{}", 
                BRIGHT_YELLOW, BOLD, test_index.map(|i| i + 1).unwrap_or(0), RESET);
        }
    }
    
    *total_errors += errors;
    *total_tested += halflen as u64;
    
    check_interruption(interrupted)
}

fn main() {
    let lang = detect_language();
    let msgs = Messages::new(&lang);
    
    let args: Vec<String> = env::args().collect();
    
    println!("{}{}{}", BRIGHT_WHITE, BOLD, msgs.title);
    println!("{}{}", BRIGHT_WHITE, msgs.author);
    println!("{}{}{}", BRIGHT_WHITE, msgs.website, RESET);
    println!();
    
    if args.len() < 2 {
        println!("{}{}{}{}", BRIGHT_YELLOW, BOLD, msgs.usage, RESET);
        return;
    }
    
    let arg = &args[1];
    let loop_mode = args.contains(&"--loop".to_string());
    let ultra_mode = args.contains(&"--ultra".to_string());
    let mut test_index = 0;
    
    // Parse test selection
    for i in 2..args.len() {
        if args[i] == "--test" && i + 1 < args.len() {
            if let Ok(idx) = args[i + 1].parse::<usize>() {
                if idx > 0 && (idx <= ALL_TESTS.len()) {
                    test_index = idx;
                }
            }
        }
    }
    
    let (total_memory, available_mem) = get_available_memory();
    
    let size = if arg.to_uppercase() == "MAX" {
        available_mem as usize
    } else {
        let unit = arg.chars().last().unwrap_or(' ').to_ascii_uppercase();
        let value_str = if arg.len() > 1 && !arg.chars().last().unwrap().is_ascii_digit() { 
            &arg[..arg.len()-1] 
        } else { 
            arg 
        };
        let value = value_str.parse::<u64>().unwrap_or(0);
        
        match unit {
            'M' => (value * 1024 * 1024) as usize,
            'G' => (value * 1024 * 1024 * 1024) as usize,
            _ => {
                println!("{}{}{}{}", BRIGHT_RED, BOLD, msgs.memory_unit_error, RESET);
                return;
            }
        }
    };
    
    // Check if we have enough memory
    if size > available_mem as usize {
        println!("{}{}{}{}\n", BRIGHT_RED, BOLD, msgs.swap_warning, RESET);
    }
    
    let actual_test_size = size / 2;
    
    if total_memory > 0 {
        println!("{}{}{:.1} MB.{}", 
            BRIGHT_WHITE, msgs.total_memory, total_memory as f64 / (1024.0 * 1024.0), RESET);
        println!("{}{}{} bytes ({} MB){}", 
            BRIGHT_WHITE, msgs.available_memory, 
            available_mem, 
            available_mem / (1024 * 1024),
            RESET);
        println!("{}{}{} bytes ({} MB) will be tested (buffer split in half){}", 
            BRIGHT_WHITE, "Memory to test: ", 
            actual_test_size, 
            actual_test_size / (1024 * 1024),
            RESET);
    } else {
        println!("Could not determine system memory. Assuming {} bytes available.", available_mem);
        println!("{} bytes will be tested (buffer split in half)", actual_test_size);
    }
    println!();
    
    // Display available tests
    println!("{}{}{}", BRIGHT_CYAN, BOLD, msgs.test_list);
    for (i, test) in ALL_TESTS.iter().enumerate() {
        // Check if this test is in DEFAULT_TESTS
        let is_default = DEFAULT_TESTS.iter().any(|t| t.name == test.name);
        let marker = if !ultra_mode && test_index == 0 && is_default { 
            " [DEFAULT]" 
        } else { 
            "" 
        };
        println!("  {}{}{}. {}{}", BRIGHT_WHITE, i + 1, RESET, test.name, marker);
    }
    println!("{}", RESET);
    println!();
    
    let single_test_mode = test_index > 0;
    
    if single_test_mode {
        println!("{}{}{}{}{}", BRIGHT_GREEN, BOLD, msgs.test_select, ALL_TESTS[test_index - 1].name, RESET);
        println!();
    } else if ultra_mode {
        println!("{}{}{}{}", BRIGHT_CYAN, BOLD, msgs.ultra_mode, RESET);
        println!();
    } else {
        println!("{}{}{}{}", BRIGHT_CYAN, BOLD, msgs.normal_mode, RESET);
        println!();
    }
    
    let interrupted = Arc::new(AtomicBool::new(false));
    
    // Determine which tests to run
    let tests_to_run: &[Test] = if single_test_mode {
        &ALL_TESTS[test_index - 1..test_index]
    } else if ultra_mode {
        &ALL_TESTS[..]
    } else {
        DEFAULT_TESTS
    };
    
    let mut rng = rand::thread_rng();
    let mut loop_count = 0;
    let mut total_errors = 0;
    let mut total_tested = 0;
    let mut total_time = 0.0;
    
    if loop_mode {
        loop {
            let pattern = rng.gen_range(0..=255u8);
            loop_count += 1;
            
            println!("{}{}Loop #{} - Pattern: {}{}", 
                BRIGHT_CYAN, BOLD, loop_count, pattern, RESET);
            
            let current_size = if arg.to_uppercase() == "MAX" {
                let (_, avail) = get_available_memory();
                println!("{}{}{} bytes ({} MB){}", 
                    BRIGHT_WHITE, msgs.available_memory, 
                    avail, 
                    avail / (1024 * 1024),
                    RESET);
                avail as usize
            } else {
                size
            };
            
            // Run tests
            for (idx, test) in tests_to_run.iter().enumerate() {
                let was_interrupted = run_test(
                    test,
                    current_size,
                    &mut total_errors,
                    &interrupted,
                    &mut total_tested,
                    &mut total_time,
                    &msgs,
                    Some(idx),
                    Some(tests_to_run.len()),
                );
                
                if was_interrupted || check_interruption(&interrupted) {
                    break;
                }
                
                // Small pause between tests
                if idx < tests_to_run.len() - 1 {
                    thread::sleep(Duration::from_millis(500));
                }
            }
            
            if check_interruption(&interrupted) {
                break;
            }
            
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        // Run tests once
        for (idx, test) in tests_to_run.iter().enumerate() {
            let was_interrupted = run_test(
                test,
                size,
                &mut total_errors,
                &interrupted,
                &mut total_tested,
                &mut total_time,
                &msgs,
                Some(idx),
                Some(tests_to_run.len()),
            );
            
            if was_interrupted || check_interruption(&interrupted) {
                break;
            }
            
            // Small pause between tests
            if idx < tests_to_run.len() - 1 {
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
    
    println!("\n{}{}{}{}", BRIGHT_YELLOW, BOLD, msgs.summary, RESET);
    if !single_test_mode && !loop_mode {
        if ultra_mode {
            println!("{}{}{}", BRIGHT_WHITE, "ULTRA mode: all 16 tests completed", RESET);
        } else {
            println!("{}{}{}", BRIGHT_WHITE, msgs.all_tests_complete, RESET);
        }
    } else if !single_test_mode && loop_mode {
        if ultra_mode {
            println!("{}{}ULTRA mode - Loop #{}", BRIGHT_WHITE, loop_count, RESET);
        } else {
            println!("{}{}All tests - Loop #{}", BRIGHT_WHITE, loop_count, RESET);
        }
    }
    if loop_count > 0 && single_test_mode {
        println!("{}{}{}", BRIGHT_WHITE, msgs.loop_count, loop_count);
    }
    println!("{}{}{}", BRIGHT_WHITE, msgs.total_errors, total_errors);
    println!("{}{}{} bytes ({} MB)", 
        BRIGHT_WHITE, msgs.total_tested, 
        total_tested, 
        total_tested / (1024 * 1024));
    
    if total_time > 0.0 {
        let avg_speed = total_tested as f64 / total_time;
        println!("{}{}{:.2}s (avg: {})", 
            BRIGHT_WHITE, msgs.total_time, total_time, format_speed(avg_speed));
    } else {
        println!("{}{}{:.2}s{}", BRIGHT_WHITE, msgs.total_time, total_time, RESET);
    }
    println!();
    
    if check_interruption(&interrupted) {
        println!("{}{}{}{}", BRIGHT_RED, BOLD, msgs.interrupted, RESET);
    }
}

