use std::env;
use std::fs;
use std::io::{self, Write, Read};
use std::time::{Instant, Duration};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Ajouter l'import manquant pour rand
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

fn get_available_memory() -> (u64, u64) {
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
        
        let adjusted_available = (available_memory as f64 * 0.95) as u64;
        (total_memory, adjusted_available)
    } else {
        (0, 0)
    }
}

fn print_progress(progress: f32, elapsed: f64, phase: &str) {
    let remaining = if progress > 0.0 {
        (elapsed / progress as f64) * (100.0 - progress as f64)
    } else {
        0.0
    };
    
    // Codes couleur ANSI
    const CYAN: &str = "\x1b[36m";
    const MAGENTA: &str = "\x1b[35m";
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";
    
    print!("\r[");
    print!("{}", CYAN);
    print!("{:6.2}%", progress);
    print!("{}", RESET);
    print!("] ");
    print!("{}", MAGENTA);
    print!("{}", phase);
    print!("{}", RESET);
    print!(" - Elapsed: ");
    print!("{}", YELLOW);
    print!("{:6.0}", elapsed);
    print!("{}", RESET);
    print!("s - Remaining: ");
    print!("{}", YELLOW);
    print!("{:6.0}", remaining);
    print!("{}s   ", RESET);
    
    io::stdout().flush().unwrap();
}

fn show_error_map(mem_errors: &[bool], total_size: usize, msgs: &Messages) {
    const MAP_WIDTH: usize = 70;
    const MAP_HEIGHT: usize = 20;
    
    let errors_count = mem_errors.iter().filter(|&&x| x).count();
    
    if errors_count == 0 {
        println!("\x1b[32mNo errors to display.\x1b[0m");
        return;
    }
    
    let blocks_per_char = (total_size / (MAP_WIDTH * MAP_HEIGHT)).max(1);
    let block_size = blocks_per_char;
    
    println!("\n\x1b[33m{} ({} {})\x1b[0m", msgs.error_map, errors_count, msgs.total_errors);
    println!("\x1b[33m{}{} bytes\x1b[0m", msgs.block_size, block_size);
    println!("\x1b[33m+{}+\x1b[0m", "-".repeat(MAP_WIDTH));
    
    for y in 0..MAP_HEIGHT {
        print!("\x1b[33m|\x1b[0m");
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
                print!("\x1b[31m#\x1b[0m");
            } else {
                print!("\x1b[32m.\x1b[0m");
            }
        }
        println!("\x1b[33m|\x1b[0m");
    }
    
    println!("\x1b[33m+{}+\x1b[0m", "-".repeat(MAP_WIDTH));
    println!("\x1b[33m{}\x1b[0m", msgs.press_esc);
    
    // Lecture simple de l'entrée clavier
    let mut stdin = io::stdin(); // CHANGÉ: déclaré comme mutable
    let mut buffer = [0u8; 1];
    loop {
        // Essayer de lire un caractère sans bloquer
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
    
    // Allouer la mémoire
    let mut memory = vec![0u8; size];
    
    // Remplissage
    println!("\x1b[34m{}\x1b[0m", msgs.filling_memory);
    
    let phase_start = Instant::now();
    
    for i in 0..size {
        memory[i] = ((i as u64 ^ pattern as u64) % 256) as u8;
        
        if i % update_interval == 0 && i > 0 {
            let progress = (i as f32 / size as f32) * 100.0;
            let elapsed = phase_start.elapsed().as_secs_f64();
            print_progress(progress, elapsed, &msgs.filling_memory);
            
            if check_interruption(interrupted) {
                println!("\n\x1b[31m{}\x1b[0m", msgs.interrupted);
                return true;
            }
        }
    }
    
    if !check_interruption(interrupted) {
        *test_duration += phase_start.elapsed().as_secs_f64();
        print_progress(100.0, phase_start.elapsed().as_secs_f64(), &msgs.filling_memory);
        println!("\n\x1b[32m{}{:.2}s\x1b[0m", msgs.filling_complete, phase_start.elapsed().as_secs_f64());
    }
    
    // Vérification
    if !check_interruption(interrupted) {
        println!("\n\x1b[34m{}\x1b[0m", msgs.verifying_memory);
        
        let phase_start = Instant::now();
        
        for i in 0..size {
            let expected_value = ((i as u64 ^ pattern as u64) % 256) as u8;
            if memory[i] != expected_value {
                errors += 1;
                mem_errors[i] = true;
                println!("\n\x1b[31m{}0x{:016x} (got:{} vs exp:{})\x1b[0m", 
                    msgs.error_at_address, 
                    &memory[i] as *const u8 as usize,
                    memory[i], 
                    expected_value);
            }
            
            if i % update_interval == 0 && i > 0 {
                let progress = (i as f32 / size as f32) * 100.0;
                let elapsed = phase_start.elapsed().as_secs_f64();
                print_progress(progress, elapsed, &msgs.verifying_memory);
                
                if check_interruption(interrupted) {
                    println!("\n\x1b[31m{}\x1b[0m", msgs.interrupted);
                    return true;
                }
            }
        }
        
        if !check_interruption(interrupted) {
            *test_duration += phase_start.elapsed().as_secs_f64();
            print_progress(100.0, phase_start.elapsed().as_secs_f64(), &msgs.verifying_memory);
            println!("\n\x1b[32m{}{:.2}s\x1b[0m", msgs.verification_complete, phase_start.elapsed().as_secs_f64());
        }
    }
    
    // Résultats
    if errors == 0 {
        println!("\n\x1b[32m{}\x1b[0m", msgs.test_success);
    } else {
        println!("\n\x1b[31m{}{} errors.\x1b[0m", msgs.test_failed, errors);
        show_error_map(&mem_errors, size, msgs);
    }
    
    *total_errors += errors;
    *total_tested += size as u64;
    
    check_interruption(interrupted)
}

fn main() {
    let lang = detect_language();
    let msgs = Messages::new(&lang);
    
    let args: Vec<String> = env::args().collect();
    
    println!("{}", msgs.title);
    println!("{}", msgs.author);
    println!("{}", msgs.website);
    println!();
    
    if args.len() < 2 {
        println!("{}", msgs.usage);
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
                println!("\x1b[31m{}\x1b[0m", msgs.memory_unit_error);
                return;
            }
        }
    };
    
    if arg.to_uppercase() != "MAX" && size > available_mem as usize {
        println!("\x1b[31m{}\x1b[0m\n", msgs.swap_warning);
    }
    
    println!("{}{:.1} MB.", msgs.total_memory, total_memory as f64 / (1024.0 * 1024.0));
    println!("{}{} bytes ({} MB)", 
        msgs.available_memory, 
        available_mem, 
        available_mem / (1024 * 1024));
    println!();
    
    let interrupted = Arc::new(AtomicBool::new(false));
    
    let mut rng = rand::thread_rng();
    let mut loop_count = 0;
    let mut total_errors = 0;
    let mut total_tested = 0;
    let mut total_time = 0.0;
    
    if loop_mode {
        loop {
            // CORRIGÉ: utilisation correcte de gen_range
            let pattern = rng.gen_range(0..=255u8);
            loop_count += 1;
            println!("Loop #{} - Pattern: {}", loop_count, pattern);
            
            let current_size = if arg.to_uppercase() == "MAX" {
                let (_, avail) = get_available_memory();
                println!("{}{} bytes ({} MB)", 
                    msgs.available_memory, 
                    avail, 
                    avail / (1024 * 1024));
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
    
    println!("\n\x1b[33m{}\x1b[0m", msgs.summary);
    if loop_count > 0 {
        println!("{}{}", msgs.loop_count, loop_count);
    }
    println!("{}{}", msgs.total_errors, total_errors);
    println!("{}{} bytes ({} MB)", 
        msgs.total_tested, 
        total_tested, 
        total_tested / (1024 * 1024));
    println!("{}{:.2}s", msgs.total_time, total_time);
    println!();
    
    if check_interruption(&interrupted) {
        println!("\x1b[31m{}\x1b[0m", msgs.interrupted);
    }
}
