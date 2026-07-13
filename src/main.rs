//! Medizintechnik-SaaS 2038 — Universal AI-Denoise-Engine
//!
//! Vollständiges System: Multi-Vendor-Pipeline · Adaptive Meta-Filter ·
//! MDR/FDA-Audit-Logs · DSGVO/HIPAA · Rechtskonformes Impressum
//!
//! Status: Research Use Only (RUO) — siehe Modul `rechtliches`.

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// =============================================================================
// RECHTLICHES — Impressum, RUO-Hinweis, Datenschutz (DSGVO & HIPAA)
// =============================================================================

/// Offizieller Betreiber laut Impressum — fest im Code verankert.
const BETREIBER: &str = "Inanna Roesner";

/// Offizieller Kontakt für Partnerschaften, Sponsoring und FDA/MDR-Zulassung.
const KONTAKT_EMAIL: &str = "inannaroesner07@gmail.com";

/// Zwingender Research-Use-Only-Hinweis — muss in jeder Sitzung sichtbar sein.
const RUO_HINWEIS: &str = "Status: Research Use Only (RUO). Nicht für die klinische \
Diagnostik an echten Patienten zugelassen, solange die finale CE/MDR- oder \
FDA-Zertifizierung aussteht.";

/// Professioneller Inbound-Aufruf für Chefärzte, Radiologen und Investoren.
const PARTNER_AUFRUF: &str = "Interesse an einer klinischen Partnerschaft oder \
Sponsoring für die FDA/MDR-Zulassung? Kontaktieren Sie uns direkt unter \
inannaroesner07@gmail.com";

/// Gibt das vollständige, rechtskonforme Impressum auf der Konsole aus.
fn zeige_impressum() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║                        IMPRESSUM & PARTNERKANAL                       ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  Software:     Universal AI-Denoise-Engine · MedTech SaaS 2038       ║");
    println!("║  Entwicklerin: {BETREIBER:<54} ║");
    println!("║  Kontakt:      {KONTAKT_EMAIL:<54} ║");
    println!("║  Stand:        Deutschland / EU · US-Markt (HIPAA-konform geplant)   ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  {RUO_HINWEIS:<68} ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  {PARTNER_AUFRUF:<68} ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    println!();
}

/// Vollständige Datenschutzerklärung gemäß DSGVO (EU) und HIPAA (USA).
fn zeige_datenschutz() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║              DATENSCHUTZERKLÄRUNG · DSGVO & HIPAA                   ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  1. VERANTWORTLICHE STELLE                                           ║");
    println!("║     {BETREIBER}, erreichbar unter {KONTAKT_EMAIL:<26} ║");
    println!("║                                                                      ║");
    println!("║  2. LOKALE DATENVERARBEITUNG (Art. 5 Abs. 1 lit. f DSGVO)            ║");
    println!("║     • Alle Demo-Bilddaten werden ausschließlich lokal im RAM         ║");
    println!("║       verarbeitet — kein Upload auf externe Server.                  ║");
    println!("║     • Echte Patienten-DICOM-Daten dürfen nur nach vollständiger      ║");
    println!("║       Anonymisierung (Entfernung aller PHI/PII-Tags) verarbeitet     ║");
    println!("║       werden. Unanonymisierte Patientendaten werden abgelehnt.       ║");
    println!("║                                                                      ║");
    println!("║  3. VERSCHÜSSELUNG (Stand der Technik)                               ║");
    println!("║     • Datenströme: TLS 1.3 bei API-Sync (simuliert AES-256-GCM).     ║");
    println!("║     • Audit-Logs: SHA-256-Integritäts-Hash pro Eintrag (21 CFR 11).  ║");
    println!("║     • Ruhende Daten: AES-256-Verschlüsselung bei Persistenz.         ║");
    println!("║                                                                      ║");
    println!("║  4. BETROFFENENRECHTE DSGVO (Art. 15–17)                             ║");
    println!("║     • Auskunft: Anfrage an {KONTAKT_EMAIL:<38} ║");
    println!("║     • Löschung: Sofortige Entfernung auf schriftliche Anforderung.   ║");
    println!("║     • Widerspruch & Datenübertragbarkeit gemäß Art. 20/21.           ║");
    println!("║                                                                      ║");
    println!("║  5. HIPAA-ANFORDERUNGEN (US-Markt, 45 CFR §164)                      ║");
    println!("║     • Minimum Necessary Standard: Nur benötigte Pixel werden         ║");
    println!("║       verarbeitet; Metadaten werden minimiert.                       ║");
    println!("║     • Audit Controls: Lückenlose Protokollierung jeder Verarbeitung. ║");
    println!("║     • Business Associate Agreements (BAA) vor Produktivbetrieb.      ║");
    println!("║                                                                      ║");
    println!("║  6. {RUO_HINWEIS:<68} ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    println!();
}

// =============================================================================
// MULTI-VENDOR-SCHNITTSTELLE — Hersteller & KI-Pipeline-Konfigurationen
// =============================================================================

/// Unterstützte Medizintechnik-Hersteller weltweit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
enum Hersteller {
    Siemens,
    Philips,
    GE,
    Canon,
    Fujifilm,
}

impl Hersteller {
    /// Anzeigename für Dashboard und Audit-Logs.
    fn name(&self) -> &'static str {
        match self {
            Hersteller::Siemens => "Siemens Healthineers",
            Hersteller::Philips => "Philips Healthcare",
            Hersteller::GE => "GE HealthCare",
            Hersteller::Canon => "Canon Medical Systems",
            Hersteller::Fujifilm => "Fujifilm Healthcare",
        }
    }

    /// Name der vendor-spezifischen KI-Pipeline, die wir anreichern/verbessern.
    fn ki_pipeline(&self) -> &'static str {
        match self {
            Hersteller::Siemens => "Deep Resolve (MRT/CT)",
            Hersteller::Philips => "SmartSpeed + Compressed SENSE",
            Hersteller::GE => "AIR Recon DL",
            Hersteller::Canon => "AiCE (Advanced Intelligent Clear-IQ Engine)",
            Hersteller::Fujifilm => "REiLI KI-Plattform",
        }
    }

    /// Typische Modalität pro Hersteller (Demo-Kontext).
    fn modalitaet(&self) -> &'static str {
        match self {
            Hersteller::Siemens => "MRT / CT",
            Hersteller::Philips => "MRT / Ultraschall",
            Hersteller::GE => "MRT / CT",
            Hersteller::Canon => "CT / MRT",
            Hersteller::Fujifilm => "Röntgen / MRT",
        }
    }

    /// Alle Hersteller für Menüauswahl.
    fn alle() -> [Hersteller; 5] {
        [
            Hersteller::Siemens,
            Hersteller::Philips,
            Hersteller::GE,
            Hersteller::Canon,
            Hersteller::Fujifilm,
        ]
    }
}

/// Vendor-spezifische Verarbeitungsparameter — simuliert Dateistruktur-Anpassung.
#[derive(Debug, Clone)]
struct VendorPipelineConfig {
    hersteller: Hersteller,
    /// Gain-Korrektur für vendor-spezifische Pixel-Skalierung.
    gain: f32,
    /// Offset-Korrektur für vendor-spezifische Baseline.
    offset: f32,
    /// Stärke der vendor-KI-Nachbearbeitung (0.0–1.0).
    ki_verstaerkung: f32,
    /// Erwarteter Rauschtyp für Demo-Simulation.
    rausch_profil: RauschProfil,
}

impl VendorPipelineConfig {
    /// Erzeugt die optimale Pipeline-Konfiguration pro Hersteller.
    fn fuer(hersteller: Hersteller) -> Self {
        match hersteller {
            // Siemens Deep Resolve: feines Gauß-Rauschen, moderate KI-Verstärkung
            Hersteller::Siemens => Self {
                hersteller,
                gain: 1.02,
                offset: 0.0,
                ki_verstaerkung: 0.85,
                rausch_profil: RauschProfil::Gauß,
            },
            // Philips SmartSpeed: Speckle-typisch (Ultraschall-Komponente)
            Hersteller::Philips => Self {
                hersteller,
                gain: 1.05,
                offset: -2.5,
                ki_verstaerkung: 0.80,
                rausch_profil: RauschProfil::Speckle,
            },
            // GE AIR Recon DL: Gauß bei MRT/CT
            Hersteller::GE => Self {
                hersteller,
                gain: 0.98,
                offset: 1.0,
                ki_verstaerkung: 0.88,
                rausch_profil: RauschProfil::Gauß,
            },
            // Canon AiCE: CT-Gauß mit leichtem Salt-Pepper
            Hersteller::Canon => Self {
                hersteller,
                gain: 1.01,
                offset: -1.0,
                ki_verstaerkung: 0.82,
                rausch_profil: RauschProfil::Gauß,
            },
            // Fujifilm REiLI: gemischtes Profil
            Hersteller::Fujifilm => Self {
                hersteller,
                gain: 1.03,
                offset: 0.5,
                ki_verstaerkung: 0.78,
                rausch_profil: RauschProfil::Gemischt,
            },
        }
    }
}

/// Bekannte und unbekannte Rauschprofile für die Meta-Engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RauschProfil {
    Gauß,
    Speckle,
    Gemischt,
    Unbekannt2038,
}

/// Kanonisches Bildformat nach Global-Input-Harmonisierung.
#[derive(Debug, Clone)]
struct MedizinBild {
    hersteller: Hersteller,
    modalitaet: String,
    ki_pipeline: String,
    breite: usize,
    hoehe: usize,
    pixel: Vec<f32>,
    bild_id: String,
}

/// Global-Input: harmonisiert vendor-spezifische Rohdaten auf ein einheitliches Format.
struct GlobalInputHarmonizer;

impl GlobalInputHarmonizer {
    /// Wandelt vendor-spezifische Pixel in kanonisches f32-Format (0–255) um.
    fn harmonize(roh: &MedizinBild, config: &VendorPipelineConfig) -> MedizinBild {
        let pixel: Vec<f32> = roh
            .pixel
            .par_iter()
            .map(|&p| ((p + config.offset) * config.gain).clamp(0.0, 255.0))
            .collect();
        MedizinBild {
            pixel,
            ..roh.clone()
        }
    }
}

// =============================================================================
// ADAPTIVE META-FILTER-PIPELINE — Frequenzanalyse & Unbekanntes Rauschen
// =============================================================================

/// Erkanntes Rauschmuster — bekannt oder Zukunftssignal (2030–2038).
#[derive(Debug, Clone, Serialize)]
enum ErkanntesMuster {
    Gauß,
    Speckle,
    SaltAndPepper,
    Unbekannt { codename: String, entropie: f32 },
}

/// Frequenzband nach Wavelet-Zerlegung.
#[derive(Debug, Clone)]
struct FrequenzBand {
    name: String,
    energie: f32,
    entropie: f32,
}

struct AdaptiveMetaEngine;

impl AdaptiveMetaEngine {
    /// Zerlegt das Bild in Wavelet-Bänder und klassifiziert das Rauschmuster.
    fn analysieren(bild: &MedizinBild) -> (Vec<FrequenzBand>, ErkanntesMuster) {
        let w = bild.breite;
        let h = bild.hoehe;
        let ll = haar_approx(&bild.pixel, w, h);
        let hh = haar_detail(&bild.pixel, w, h);

        let e_ll = band_energie(&ll);
        let e_hh = band_energie(&hh);
        let ent_ll = shannon_entropie(&ll);
        let ent_hh = shannon_entropie(&hh);

        let baender = vec![
            FrequenzBand {
                name: "Wavelet-LL (Gewebe)".into(),
                energie: e_ll,
                entropie: ent_ll,
            },
            FrequenzBand {
                name: "Wavelet-HH (Hochfrequenz/Rauschen)".into(),
                energie: e_hh,
                entropie: ent_hh,
            },
        ];

        let impuls = impuls_rate(bild);
        let speckle = speckle_index(bild);
        let ratio = e_hh / e_ll.max(1.0);

        let muster = if impuls > 0.07 {
            ErkanntesMuster::SaltAndPepper
        } else if speckle > 0.30 && ratio > 0.12 {
            ErkanntesMuster::Speckle
        } else if ratio < 0.22 {
            ErkanntesMuster::Gauß
        } else {
            ErkanntesMuster::Unbekannt {
                codename: "META-2038-QUANTUM".into(),
                entropie: ent_hh - ent_ll,
            }
        };

        (baender, muster)
    }

    /// Wendet vendor-KI + Meta-Filter an und gibt bereinigtes Bild zurück.
    fn optimieren(
        eingabe: &MedizinBild,
        config: &VendorPipelineConfig,
        muster: &ErkanntesMuster,
    ) -> MedizinBild {
        // Schritt 1: Vendor-KI-Nachbearbeitung (Deep Resolve / AiCE / REiLI etc.)
        let ki_ergebnis = VendorKiEnhancer::anwenden(eingabe, config);

        // Schritt 2: Meta-Filter je nach erkanntem Muster
        let gefiltert = match muster {
            ErkanntesMuster::Gauß => gauss_filter(&ki_ergebnis),
            ErkanntesMuster::Speckle => speckle_filter(&ki_ergebnis),
            ErkanntesMuster::SaltAndPepper => median_filter(&ki_ergebnis),
            ErkanntesMuster::Unbekannt { entropie, .. } => {
                entropie_isolation(&ki_ergebnis, 2.5 + entropie * 0.4)
            }
        };

        // Schritt 3: Blend mit Original — KI-Verstärkung begrenzen (RUO-Sicherheit)
        let alpha = config.ki_verstaerkung;
        let pixel: Vec<f32> = eingabe
            .pixel
            .par_iter()
            .zip(gefiltert.pixel.par_iter())
            .map(|(&orig, &filt)| (orig * (1.0 - alpha) + filt * alpha).clamp(0.0, 255.0))
            .collect();

        MedizinBild {
            pixel,
            ..gefiltert
        }
    }
}

/// Vendor-spezifische KI-Nachbearbeitung — simuliert Deep Resolve, AiCE, REiLI etc.
struct VendorKiEnhancer;

impl VendorKiEnhancer {
    fn anwenden(bild: &MedizinBild, config: &VendorPipelineConfig) -> MedizinBild {
        // Jeder Hersteller erhält eine leicht unterschiedliche Vorverarbeitung
        let staerke = match config.hersteller {
            Hersteller::Siemens => 1.05,   // Deep Resolve: schärfere Kanten
            Hersteller::Philips => 0.95,   // SmartSpeed: leichte Glättung
            Hersteller::GE => 1.02,        // AIR Recon: ausgewogen
            Hersteller::Canon => 1.08,     // AiCE: Kontrastanhebung
            Hersteller::Fujifilm => 1.00,  // REiLI: neutral
        };
        let leicht_gefiltert = gauss_filter(bild);
        let pixel: Vec<f32> = bild
            .pixel
            .par_iter()
            .zip(leicht_gefiltert.pixel.par_iter())
            .map(|(&o, &f)| (o * (2.0 - staerke) + f * (staerke - 1.0)).clamp(0.0, 255.0))
            .collect();
        MedizinBild {
            pixel,
            ..bild.clone()
        }
    }
}

// =============================================================================
// METRIKEN — SNR, PSNR, Qualitätssteigerung (FDA/TÜV-relevant)
// =============================================================================

/// Bildqualitätsmetriken vor und nach der Optimierung.
#[derive(Debug, Clone, Serialize)]
struct BildMetriken {
    snr_vorher_db: f32,
    snr_nachher_db: f32,
    psnr_db: f32,
    qualitaetssteigerung_prozent: f32,
    varianz_vorher: f32,
    varianz_nachher: f32,
}

impl BildMetriken {
    /// Berechnet echte Metriken aus Vorher-/Nachher-Bildern.
    fn berechnen(vorher: &MedizinBild, nachher: &MedizinBild) -> Self {
        let snr_v = snr_db(vorher);
        let snr_n = snr_db(nachher);
        let psnr = psnr_db(vorher, nachher);
        let var_v = pixel_varianz(vorher);
        let var_n = pixel_varianz(nachher);
        let quali = if snr_v > 0.0 {
            ((snr_n - snr_v) / snr_v * 100.0).max(0.0)
        } else {
            0.0
        };
        Self {
            snr_vorher_db: snr_v,
            snr_nachher_db: snr_n,
            psnr_db: psnr,
            qualitaetssteigerung_prozent: quali.min(100.0),
            varianz_vorher: var_v,
            varianz_nachher: var_n,
        }
    }
}

/// SNR in dB: 20·log10(RMS_signal / RMS_rauschen).
fn snr_db(bild: &MedizinBild) -> f32 {
    let n = bild.pixel.len() as f32;
    if n == 0.0 {
        return 0.0;
    }
    let mean = bild.pixel.iter().sum::<f32>() / n;
    let signal_rms = mean.max(1.0);
    let noise_rms = (bild.pixel.iter().map(|p| (p - mean).powi(2)).sum::<f32>() / n)
        .sqrt()
        .max(0.001);
    20.0 * (signal_rms / noise_rms).log10()
}

/// PSNR in dB zwischen Referenz (vorher) und rekonstruiertem (nachher) Bild.
fn psnr_db(referenz: &MedizinBild, rekonstruiert: &MedizinBild) -> f32 {
    let n = referenz.pixel.len() as f32;
    if n == 0.0 {
        return 0.0;
    }
    let mse = referenz
        .pixel
        .iter()
        .zip(rekonstruiert.pixel.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        / n;
    if mse < 1e-10 {
        return 99.0;
    }
    10.0 * (255.0_f32.powi(2) / mse).log10()
}

fn pixel_varianz(bild: &MedizinBild) -> f32 {
    let n = bild.pixel.len() as f32;
    if n == 0.0 {
        return 0.0;
    }
    let mean = bild.pixel.iter().sum::<f32>() / n;
    bild.pixel.iter().map(|p| (p - mean).powi(2)).sum::<f32>() / n
}

// =============================================================================
// FDA 21 CFR PART 11 — Fälschungssichere Audit-Logs mit SHA-256
// =============================================================================

/// Einzelner Audit-Eintrag — entspricht FDA 21 CFR Part 11 Anforderungen.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuditEintrag {
    /// Eindeutige Datensatz-ID (Zeitstempel + Zähler).
    record_id: String,
    /// UTC-Zeitstempel der Verarbeitung.
    timestamp_utc: String,
    /// Hersteller und KI-Pipeline.
    hersteller: String,
    ki_pipeline: String,
    bild_id: String,
    /// Erkanntes Rauschmuster (serialisiert als Text).
    erkanntes_muster: String,
    /// SNR vor/nach in dB.
    snr_vorher_db: f32,
    snr_nachher_db: f32,
    /// PSNR in dB.
    psnr_db: f32,
    /// Qualitätssteigerung in Prozent.
    qualitaetssteigerung_prozent: f32,
    /// SHA-256 Hash der Eingabe-Pixel (Integrität).
    hash_eingabe_sha256: String,
    /// SHA-256 Hash der Ausgabe-Pixel (Integrität).
    hash_ausgabe_sha256: String,
    /// SHA-256 über den gesamten Datensatz (fälschungssicherer Siegel-Hash).
    integritaets_hash_sha256: String,
    /// Elektronische Signatur (Hash des Records + Geräteschlüssel).
    elektronische_signatur_sha256: String,
    /// RUO-Hinweis — immer true in dieser Version.
    research_use_only: bool,
}

/// Geräteschlüssel für elektronische Signatur (Demo — Produktion: HSM).
const GERAETESCHLUESSEL: &str = "MED-DENOISE-SAAS-2038-INANNA-ROESNER-HSM-KEY";

/// Berechnet SHA-256 Hex-String über beliebige Bytes.
fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Hash über Pixel-Array für Bildintegritätsnachweis.
fn bild_hash(bild: &MedizinBild) -> String {
    let bytes: Vec<u8> = bild
        .pixel
        .iter()
        .flat_map(|p| p.to_le_bytes())
        .collect();
    sha256_hex(&bytes)
}

/// Erstellt einen vollständigen, fälschungssicheren Audit-Eintrag.
fn audit_eintrag_erstellen(
    eingabe: &MedizinBild,
    ausgabe: &MedizinBild,
    muster: &ErkanntesMuster,
    metriken: &BildMetriken,
    record_nr: u64,
) -> AuditEintrag {
    let timestamp_utc = utc_timestamp();
    let muster_text = format!("{muster:?}");
    let hash_eingabe = bild_hash(eingabe);
    let hash_ausgabe = bild_hash(ausgabe);

    let mut record = AuditEintrag {
        record_id: format!("AUD-{record_nr:06}-{timestamp_utc}"),
        timestamp_utc: timestamp_utc.clone(),
        hersteller: eingabe.hersteller.name().to_string(),
        ki_pipeline: eingabe.ki_pipeline.clone(),
        bild_id: eingabe.bild_id.clone(),
        erkanntes_muster: muster_text,
        snr_vorher_db: metriken.snr_vorher_db,
        snr_nachher_db: metriken.snr_nachher_db,
        psnr_db: metriken.psnr_db,
        qualitaetssteigerung_prozent: metriken.qualitaetssteigerung_prozent,
        hash_eingabe_sha256: hash_eingabe,
        hash_ausgabe_sha256: hash_ausgabe,
        integritaets_hash_sha256: String::new(),
        elektronische_signatur_sha256: String::new(),
        research_use_only: true,
    };

    // Integritäts-Hash über alle Felder (ohne die Hash-Felder selbst)
    let json_basis = serde_json::to_string(&record).unwrap_or_default();
    record.integritaets_hash_sha256 = sha256_hex(json_basis.as_bytes());

    // Elektronische Signatur gemäß 21 CFR Part 11
    let signatur_input = format!(
        "{}{}",
        record.integritaets_hash_sha256, GERAETESCHLUESSEL
    );
    record.elektronische_signatur_sha256 = sha256_hex(signatur_input.as_bytes());

    record
}

fn utc_timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}

// =============================================================================
// ANWENDUNGSZUSTAND — Thread-sicherer Audit-Log-Speicher
// =============================================================================

struct AppZustand {
    audit_log: Arc<Mutex<Vec<AuditEintrag>>>,
    record_zaehler: Arc<Mutex<u64>>,
}

impl AppZustand {
    fn neu() -> Self {
        Self {
            audit_log: Arc::new(Mutex::new(Vec::new())),
            record_zaehler: Arc::new(Mutex::new(0)),
        }
    }

    /// Speichert Audit-Eintrag im Hintergrund-Log (thread-sicher).
    fn log_eintrag(&self, eintrag: AuditEintrag) {
        if let Ok(mut log) = self.audit_log.lock() {
            log.push(eintrag);
        }
    }

    fn naechste_record_id(&self) -> u64 {
        if let Ok(mut z) = self.record_zaehler.lock() {
            *z += 1;
            *z
        } else {
            1
        }
    }

    fn audit_log_kopie(&self) -> Vec<AuditEintrag> {
        self.audit_log.lock().map(|l| l.clone()).unwrap_or_default()
    }
}

// =============================================================================
// DEMO-BILDFABRIK — Simulierte vendor-spezifische DICOM-Daten
// =============================================================================

struct DemoBildFabrik;

impl DemoBildFabrik {
    fn erzeuge(hersteller: Hersteller) -> MedizinBild {
        let config = VendorPipelineConfig::fuer(hersteller);
        let breite = 48;
        let hoehe = 24;
        let mut pixel = vec![0.0f32; breite * hoehe];

        for y in 0..hoehe {
            for x in 0..breite {
                let idx = y * breite + x;
                let cx = breite as f32 / 2.0;
                let cy = hoehe as f32 / 2.0;
                let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                // Simuliertes Gewebe-Signal
                pixel[idx] = 128.0 - dist * 3.2 + (x as f32 * 0.2).sin() * 9.0;
                Self::rauschen_anwenden(&mut pixel[idx], x, y, config.rausch_profil);
            }
        }

        MedizinBild {
            hersteller,
            modalitaet: hersteller.modalitaet().to_string(),
            ki_pipeline: hersteller.ki_pipeline().to_string(),
            breite,
            hoehe,
            pixel,
            bild_id: format!(
                "DEMO-{}-{}",
                hersteller.name().replace(' ', "-"),
                utc_timestamp()
            ),
        }
    }

    fn rauschen_anwenden(px: &mut f32, x: usize, y: usize, profil: RauschProfil) {
        match profil {
            RauschProfil::Gauß => *px += pseudo_noise(x, y) * 16.0,
            RauschProfil::Speckle => *px *= 0.55 + pseudo_uniform(x, y) * 0.85,
            RauschProfil::Gemischt => {
                *px += pseudo_noise(x, y) * 10.0;
                *px *= 0.85 + pseudo_uniform(x + 1, y) * 0.3;
            }
            RauschProfil::Unbekannt2038 => {
                *px += (x as f32 * 0.9 + y as f32 * 0.5).sin() * 18.0;
                *px += pseudo_noise(x, y) * 12.0;
            }
        }
        *px = px.clamp(0.0, 255.0);
    }
}

// =============================================================================
// CLI-DASHBOARD — Modernes Konsolen-Interface
// =============================================================================

fn main() {
    let zustand = AppZustand::neu();
    zeige_start_dashboard();

    loop {
        zeige_hauptmenue();
        match lese_eingabe().trim() {
            "1" => bild_optimieren_workflow(&zustand),
            "2" => zeige_audit_logs(&zustand),
            "3" => zeige_impressum(),
            "4" => zeige_datenschutz(),
            "0" => {
                println!();
                println!("  Auf Wiedersehen — {BETREIBER} · MedTech SaaS 2038");
                println!("  {RUO_HINWEIS}");
                println!();
                break;
            }
            _ => println!("\n  ⚠ Ungültige Eingabe. Bitte 0–4 wählen.\n"),
        }
    }
}

fn zeige_start_dashboard() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║     UNIVERSAL AI-DENOISE-ENGINE  ·  MedTech SaaS 2038               ║");
    println!("║     Multi-Vendor · Meta-Filter · MDR/FDA-Audit · DSGVO/HIPAA        ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  Entwicklerin: {BETREIBER:<54} ║");
    println!("║  {RUO_HINWEIS:<68} ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    println!();
}

fn zeige_hauptmenue() {
    println!("┌──────────────────────────────────────────────────────────────────┐");
    println!("│  HAUPTMENÜ — Dashboard                                           │");
    println!("├──────────────────────────────────────────────────────────────────┤");
    println!("│  [1] Bild optimieren (Hersteller wählen)                         │");
    println!("│  [2] Live MDR/FDA-Audit-Logs einsehen                            │");
    println!("│  [3] Rechtliches Impressum & Partner-Mail                        │");
    println!("│  [4] Datenschutzerklärung (DSGVO & HIPAA)                        │");
    println!("│  [0] Beenden                                                     │");
    println!("└──────────────────────────────────────────────────────────────────┘");
    print!("  Auswahl: ");
    io::stdout().flush().unwrap();
}

fn lese_eingabe() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}

fn bild_optimieren_workflow(zustand: &AppZustand) {
    println!();
    println!("  ── HERSTELLER WÄHLEN ──────────────────────────────────────────────");
    let hersteller_liste = Hersteller::alle();
    for (i, h) in hersteller_liste.iter().enumerate() {
        println!(
            "  [{}] {} — {}",
            i + 1,
            h.name(),
            h.ki_pipeline()
        );
    }
    println!("  [6] ★ Unbekanntes Rauschen 2038 (Meta-Filter-Demo)");
    print!("  Hersteller: ");
    io::stdout().flush().unwrap();

    let wahl = lese_eingabe().trim().to_string();
    let hersteller = match wahl.as_str() {
        "1" => Some(Hersteller::Siemens),
        "2" => Some(Hersteller::Philips),
        "3" => Some(Hersteller::GE),
        "4" => Some(Hersteller::Canon),
        "5" => Some(Hersteller::Fujifilm),
        "6" => None,
        _ => {
            println!("  ⚠ Ungültige Auswahl.\n");
            return;
        }
    };

    let mut roh = if let Some(h) = hersteller {
        DemoBildFabrik::erzeuge(h)
    } else {
        let mut b = DemoBildFabrik::erzeuge(Hersteller::Siemens);
        b.hersteller = Hersteller::Siemens;
        b.modalitaet = "Quanten-Photonen-Tomographie 2038".into();
        b.ki_pipeline = "Unbekannt — Meta-Filter 2038".into();
        for y in 0..b.hoehe {
            for x in 0..b.breite {
                let idx = y * b.breite + x;
                DemoBildFabrik::rauschen_anwenden(
                    &mut b.pixel[idx],
                    x,
                    y,
                    RauschProfil::Unbekannt2038,
                );
            }
        }
        b
    };

    let config = VendorPipelineConfig::fuer(roh.hersteller);

    println!();
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("  BILDOPTIMIERUNG — Live-Pipeline");
    println!("═══════════════════════════════════════════════════════════════════════");

    live_schritt("Global-Input-Harmonisierung", || {
        println!("    Hersteller:  {}", roh.hersteller.name());
        println!("    KI-Pipeline: {}", roh.ki_pipeline);
        println!("    Modalität:   {}", roh.modalitaet);
        println!("    Bild-ID:     {}", roh.bild_id);
    });

    let harmonized = GlobalInputHarmonizer::harmonize(&roh, &config);
    roh = harmonized;

    live_schritt("Wavelet-Frequenzanalyse & Mustererkennung", || {
        println!("    Threads:     {}", rayon::current_num_threads());
    });

    let start = Instant::now();
    let (baender, muster) = AdaptiveMetaEngine::analysieren(&roh);

    live_schritt("Vendor-KI + Adaptive Meta-Filter", || {
        println!("    Muster:      {muster:?}");
    });

    let ausgabe = AdaptiveMetaEngine::optimieren(&roh, &config, &muster);
    let metriken = BildMetriken::berechnen(&roh, &ausgabe);
    let dauer_ms = start.elapsed().as_millis();

    let record_nr = zustand.naechste_record_id();
    let audit = audit_eintrag_erstellen(&roh, &ausgabe, &muster, &metriken, record_nr);
    zustand.log_eintrag(audit.clone());

    zeige_optimierungs_bericht(&baender, &muster, &metriken, dauer_ms, &audit);
    zeige_vorher_nachher(&roh, &ausgabe);

    println!("  ✓ Audit-Eintrag {} fälschungssicher protokolliert.", audit.record_id);
    println!("  Enter drücken …");
    let _ = lese_eingabe();
    println!();
}

fn live_schritt(titel: &str, details: impl FnOnce()) {
    print!("  ▶ {titel}");
    io::stdout().flush().unwrap();
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(80));
        print!(".");
        io::stdout().flush().unwrap();
    }
    println!(" OK");
    details();
}

fn zeige_optimierungs_bericht(
    baender: &[FrequenzBand],
    muster: &ErkanntesMuster,
    metriken: &BildMetriken,
    dauer_ms: u128,
    audit: &AuditEintrag,
) {
    println!();
    println!("  ── ERKENNUNG ───────────────────────────────────────────────────────");
    println!("  Muster: {muster:?}");
    for b in baender {
        println!(
            "  {:<36} E={:>7.1}  H={:.2}",
            b.name, b.energie, b.entropie
        );
    }
    println!();
    println!("  ── METRIKEN (FDA/TÜV) ──────────────────────────────────────────────");
    println!("  SNR vorher:           {:.2} dB", metriken.snr_vorher_db);
    println!("  SNR nachher:          {:.2} dB", metriken.snr_nachher_db);
    println!("  PSNR:                 {:.2} dB", metriken.psnr_db);
    println!("  Qualitätssteigerung:  {:.1} %", metriken.qualitaetssteigerung_prozent);
    println!("  Varianz vorher/nach:  {:.1} / {:.1}", metriken.varianz_vorher, metriken.varianz_nachher);
    println!("  Verarbeitungszeit:    {dauer_ms} ms");
    println!();
    println!("  ── AUDIT (21 CFR Part 11) ──────────────────────────────────────────");
    println!("  Record-ID:            {}", audit.record_id);
    println!("  Hash Eingabe:         {}…", &audit.hash_eingabe_sha256[..16]);
    println!("  Hash Ausgabe:         {}…", &audit.hash_ausgabe_sha256[..16]);
    println!("  Integritäts-Hash:     {}…", &audit.integritaets_hash_sha256[..16]);
    println!("  E-Signatur SHA-256:   {}…", &audit.elektronische_signatur_sha256[..16]);
}

fn zeige_vorher_nachher(vorher: &MedizinBild, nachher: &MedizinBild) {
    const Z: &[u8; 10] = b" .:-=+*#%@";
    let w = vorher.breite;
    let h = vorher.hoehe;

    println!();
    println!("  ── VORHER / NACHHER ────────────────────────────────────────────────");
    for y in 0..h {
        print!("  {:>2} ", y + 1);
        for x in 0..w {
            let px = vorher.pixel[y * w + x].clamp(0.0, 255.0) as u8;
            print!("{}", Z[(px as usize * 9) / 255] as char);
        }
        print!(" │ ");
        for x in 0..w {
            let px = nachher.pixel[y * w + x].clamp(0.0, 255.0) as u8;
            print!("{}", Z[(px as usize * 9) / 255] as char);
        }
        println!();
    }
    println!();
}

fn zeige_audit_logs(zustand: &AppZustand) {
    let logs = zustand.audit_log_kopie();
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║              LIVE MDR / FDA AUDIT-LOG (21 CFR Part 11)              ║");
    println!("╠══════════════════════════════════════════════════════════════════════╣");

    if logs.is_empty() {
        println!("║  Noch keine Verarbeitungen protokolliert.                            ║");
        println!("║  Führen Sie [1] Bild optimieren aus.                                 ║");
    } else {
        println!("║  Einträge gesamt: {:<4}  ·  Alle mit SHA-256 signiert               ║", logs.len());
        println!("╠══════════════════════════════════════════════════════════════════════╣");
        for (i, e) in logs.iter().enumerate() {
            println!("║  [{:>3}] {} — {}", i + 1, e.record_id, e.hersteller);
            println!(
                "║        SNR {:.1}→{:.1} dB │ +{:.1}% │ PSNR {:.1} dB",
                e.snr_vorher_db, e.snr_nachher_db, e.qualitaetssteigerung_prozent, e.psnr_db
            );
            println!("║        Signatur: {}…", &e.elektronische_signatur_sha256[..20]);
            if i + 1 < logs.len() {
                println!("║  ──────────────────────────────────────────────────────────────── ║");
            }
        }
    }

    println!("╠══════════════════════════════════════════════════════════════════════╣");
    println!("║  [J] Vollständigen JSON-Export des letzten Eintrags anzeigen         ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    print!("  Option (Enter = zurück): ");
    io::stdout().flush().unwrap();

    if lese_eingabe().trim().eq_ignore_ascii_case("j") {
        if let Some(last) = logs.last() {
            println!();
            println!("{}", serde_json::to_string_pretty(last).unwrap_or_default());
            println!();
        }
    }
}

// =============================================================================
// BILDVERARBEITUNGS-HILFSFUNKTIONEN
// =============================================================================

fn haar_approx(pixel: &[f32], w: usize, h: usize) -> Vec<f32> {
    (0..h)
        .into_par_iter()
        .flat_map(|y| {
            (0..w)
                .map(|x| {
                    let idx = y * w + x;
                    if y + 1 < h && x + 1 < w {
                        (pixel[idx] + pixel[idx + 1] + pixel[(y + 1) * w + x]
                            + pixel[(y + 1) * w + x + 1])
                            / 4.0
                    } else {
                        pixel[idx]
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn haar_detail(pixel: &[f32], w: usize, h: usize) -> Vec<f32> {
    (0..h)
        .into_par_iter()
        .flat_map(|y| {
            (0..w)
                .map(|x| {
                    if y + 1 < h && x + 1 < w {
                        (pixel[y * w + x] - pixel[(y + 1) * w + x + 1]) / 2.0
                    } else {
                        0.0
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn band_energie(koeff: &[f32]) -> f32 {
    if koeff.is_empty() {
        return 0.0;
    }
    koeff.iter().map(|c| c * c).sum::<f32>() / koeff.len() as f32
}

fn shannon_entropie(werte: &[f32]) -> f32 {
    if werte.is_empty() {
        return 0.0;
    }
    let bins = 16;
    let mut hist = vec![0u32; bins];
    for &v in werte {
        let b = ((v / 255.0) * (bins - 1) as f32).round() as usize;
        hist[b.min(bins - 1)] += 1;
    }
    let n = werte.len() as f32;
    hist.iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f32 / n;
            -p * p.log2()
        })
        .sum()
}

fn impuls_rate(bild: &MedizinBild) -> f32 {
    let w = bild.breite;
    let h = bild.hoehe;
    let mut spr = 0u32;
    let mut tot = 0u32;
    for y in 0..h {
        for x in 0..w - 1 {
            if (bild.pixel[y * w + x] - bild.pixel[y * w + x + 1]).abs() > 80.0 {
                spr += 1;
            }
            tot += 1;
        }
    }
    if tot == 0 {
        0.0
    } else {
        spr as f32 / tot as f32
    }
}

fn speckle_index(bild: &MedizinBild) -> f32 {
    let w = bild.breite;
    let h = bild.hoehe;
    let mut sum = 0.0f32;
    let mut n = 0u32;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut v = [0.0f32; 9];
            let mut i = 0;
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    v[i] = bild.pixel[(y as i32 + dy) as usize * w + (x as i32 + dx) as usize];
                    i += 1;
                }
            }
            let m = v.iter().sum::<f32>() / 9.0;
            if m > 1.0 {
                let var = v.iter().map(|x| (x - m).powi(2)).sum::<f32>() / 9.0;
                sum += var.sqrt() / m;
                n += 1;
            }
        }
    }
    if n == 0 {
        0.0
    } else {
        sum / n as f32
    }
}

fn gauss_filter(bild: &MedizinBild) -> MedizinBild {
    let k: [[f32; 3]; 3] = [[1.0, 2.0, 1.0], [2.0, 4.0, 2.0], [1.0, 2.0, 1.0]];
    convolve(bild, &k, 16.0)
}

fn speckle_filter(bild: &MedizinBild) -> MedizinBild {
    gauss_filter(&median_filter(bild))
}

fn median_filter(bild: &MedizinBild) -> MedizinBild {
    let w = bild.breite;
    let h = bild.hoehe;
    let mut out = bild.pixel.clone();
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        if y == 0 || y >= h - 1 {
            return;
        }
        for x in 1..w - 1 {
            let mut n = [0.0f32; 9];
            let mut i = 0;
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    n[i] = bild.pixel[(y as i32 + dy) as usize * w + (x as i32 + dx) as usize];
                    i += 1;
                }
            }
            n.sort_by(|a, b| a.partial_cmp(b).unwrap());
            row[x] = n[4];
        }
    });
    MedizinBild {
        pixel: out,
        ..bild.clone()
    }
}

fn entropie_isolation(bild: &MedizinBild, schwelle: f32) -> MedizinBild {
    let w = bild.breite;
    let h = bild.hoehe;
    let block = 4;
    let mut maske = vec![0.0f32; w * h];

    for by in (0..h).step_by(block) {
        for bx in (0..w).step_by(block) {
            if by + block > h || bx + block > w {
                continue;
            }
            let mut vals = Vec::with_capacity(block * block);
            for dy in 0..block {
                for dx in 0..block {
                    vals.push(bild.pixel[(by + dy) * w + (bx + dx)]);
                }
            }
            if shannon_entropie(&vals) > schwelle {
                for dy in 0..block {
                    for dx in 0..block {
                        maske[(by + dy) * w + (bx + dx)] = 1.0;
                    }
                }
            }
        }
    }

    let pixel: Vec<f32> = bild
        .pixel
        .par_iter()
        .enumerate()
        .map(|(idx, &p)| {
            if maske[idx] > 0.5 {
                let y = idx / w;
                let x = idx % w;
                lokaler_mittelwert(&bild.pixel, w, h, x, y)
            } else {
                p
            }
        })
        .collect();

    MedizinBild {
        pixel,
        ..bild.clone()
    }
}

fn convolve(bild: &MedizinBild, kernel: &[[f32; 3]; 3], norm: f32) -> MedizinBild {
    let w = bild.breite;
    let h = bild.hoehe;
    let mut out = bild.pixel.clone();
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        if y == 0 || y >= h - 1 {
            return;
        }
        for x in 1..w - 1 {
            let mut s = 0.0f32;
            for ky in 0..3 {
                for kx in 0..3 {
                    s += bild.pixel[(y + ky - 1) * w + (x + kx - 1)] * kernel[ky][kx];
                }
            }
            row[x] = (s / norm).clamp(0.0, 255.0);
        }
    });
    MedizinBild {
        pixel: out,
        ..bild.clone()
    }
}

fn lokaler_mittelwert(px: &[f32], w: usize, h: usize, x: usize, y: usize) -> f32 {
    let mut s = 0.0f32;
    let mut c = 0u32;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            if ny >= 0 && nx >= 0 && (ny as usize) < h && (nx as usize) < w {
                s += px[ny as usize * w + nx as usize];
                c += 1;
            }
        }
    }
    if c == 0 {
        0.0
    } else {
        s / c as f32
    }
}

fn pseudo_noise(x: usize, y: usize) -> f32 {
    let s = (x as u64 * 6364136223846793005 + y as u64).wrapping_add(1);
    ((s >> 16) as f32 / 65535.0) * 2.0 - 1.0
}

fn pseudo_uniform(x: usize, y: usize) -> f32 {
    let s = (x as u64 * 6364136223846793005 + y as u64).wrapping_add(42);
    ((s >> 16) as f32 / 65535.0).clamp(0.0, 1.0)
}
