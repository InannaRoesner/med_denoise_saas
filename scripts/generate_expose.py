#!/usr/bin/env python3
"""Generiert das Sponsoring-Exposé als PDF via WeasyPrint."""

from pathlib import Path

HTML_CONTENT = """<!DOCTYPE html>
<html lang="de">
<head>
<meta charset="UTF-8">
<title>Unternehmenskonzept & Sponsoring-Exposé</title>
<style>
@page {
    size: A4;
    margin: 20mm 15mm;
    background-color: #ffffff;
    @bottom-right {
        content: "Seite " counter(page);
        font-family: 'Arial', sans-serif;
        font-size: 9pt;
        color: #718096;
    }
    @bottom-left {
        content: "Inanna Roesner — Medizintechnik-SaaS";
        font-family: 'Arial', sans-serif;
        font-size: 9pt;
        color: #718096;
    }
}

*, *::before, *::after {
    box-sizing: border-box;
}

body {
    font-family: 'Arial', sans-serif;
    font-size: 11pt;
    line-height: 1.6;
    color: #2d3748;
    margin: 0;
    padding: 0;
}

.header-banner {
    margin: -20mm -15mm 25px -15mm;
    padding: 35px 15mm;
    background-color: #0f172a;
    color: #ffffff;
}

h1 {
    font-size: 24pt;
    margin: 0 0 10px 0;
    color: #ffffff;
    font-weight: bold;
    letter-spacing: -0.5px;
}

.subtitle {
    font-size: 13pt;
    color: #38bdf8;
    margin: 0;
    font-weight: normal;
}

h2 {
    font-size: 15pt;
    color: #0f172a;
    border-left: 5px solid #0284c7;
    padding-left: 10px;
    margin-top: 30px;
    margin-bottom: 15px;
    page-break-after: avoid;
}

h3 {
    font-size: 12pt;
    color: #1e293b;
    margin-top: 20px;
    margin-bottom: 8px;
    page-break-after: avoid;
}

p {
    margin-top: 0;
    margin-bottom: 15px;
    text-align: justify;
}

ul {
    margin-top: 0;
    margin-bottom: 15px;
    padding-left: 20px;
}

li {
    margin-bottom: 8px;
}

.highlight-box {
    background-color: #f0fdf4;
    border-left: 4px solid #16a34a;
    padding: 15px;
    margin: 20px 0;
    page-break-inside: avoid;
}

.highlight-box h3 {
    color: #14532d;
    margin-top: 0;
}

.sponsor-box {
    background-color: #eff6ff;
    border-left: 4px solid #0284c7;
    padding: 15px;
    margin: 20px 0;
    page-break-inside: avoid;
}

.contact-card {
    background-color: #f8fafc;
    border: 1px solid #e2e8f0;
    padding: 20px;
    margin-top: 40px;
    page-break-inside: avoid;
}

.contact-card table {
    width: 100%;
    border-collapse: collapse;
}

.contact-card td {
    padding: 4px 0;
    vertical-align: top;
}

.contact-card td.label {
    font-weight: bold;
    width: 120px;
    color: #475569;
}

.ruo-notice {
    font-size: 9pt;
    color: #64748b;
    border-top: 1px solid #e2e8f0;
    margin-top: 30px;
    padding-top: 12px;
}
</style>
</head>
<body>

<div class="header-banner">
    <h1>Medizintechnik-SaaS Pipeline</h1>
    <div class="subtitle">Strategisches Whitepaper &amp; Sponsoring-Exposé für Krankenhäuser und Partner</div>
</div>

<p><strong>Initiatorin &amp; Chefarchitektin:</strong> Inanna Roesner<br>
<strong>Status des Systems:</strong> Research Use Only (RUO) / Bereit für strategische Partnerschaften</p>

<h2>1. Die Vision: Infinite Evolution AI Engine</h2>
<p>
Moderne medizinische Bildgebungsverfahren (MRT und CT) sind auf präzise Rauschunterdrückung angewiesen, um Artefakte zu minimieren und Scanzeiten für Patienten drastisch zu verkürzen. Die <strong>Infinite Evolution AI Pipeline</strong> bricht mit dem Paradigma starrer, fest programmierter Filter.
</p>
<p>
Architektonisch ist das System auf unendliche Zukunftssicherheit ausgelegt (<em>Infinite Upgrade Design</em>). Über eine universelle, adaptive Schnittstelle ist die Pipeline in der Lage, jede neu auf den Markt kommende Generation generativer KI-Modelle und Deep-Learning-Rekonstruktionen autonom zu integrieren. Dadurch ist gewährleistet, dass das Gesamtsystem technologisch und qualitativ dauernd an der Weltspitze operiert, ohne dass der Kerncode modifiziert werden muss.
</p>

<h2>2. Technische Roadmap &amp; Next-Gen Upgrades</h2>
<p>Das System durchläuft aktuell eine gezielte Evolution, um aus der erfolgreichen Systemsimulation ein marktreifes Hochleistungsprodukt zu formen:</p>
<ul>
    <li><strong>Echte native KI-Core-Integration:</strong> Vollständige Ablösung der algorithmischen Simulation durch die direkte Einbindung kompakter, hochperformanter On-Device-KI-Modelle (Deep Learning) zur Echtzeit-Bildoptimierung direkt auf den radiologischen Systemen.</li>
    <li><strong>Grafische Benutzeroberfläche (GUI):</strong> Entwicklung einer modernen, plattformübergreifenden Oberfläche mit Fenstern, Schiebereglern, Vorher-Nachher-Vergleichen und intuitiven Bedienelementen. Dies ersetzt das textbasierte Terminal-Dashboard vollständig und ermöglicht radiologischem Fachpersonal eine intuitive Steuerung im klinischen Alltag.</li>
    <li><strong>Dauerbetrieb-Pipeline:</strong> Umstellung der Software-Architektur auf ein unendliches Schleifen-System. Dies ermöglicht die vollautomatische Massenverarbeitung kontinuierlicher medizinischer Bilddatenströme im 24/7-Schichtbetrieb von Großkliniken.</li>
</ul>

<h2>3. Rechtliche Absicherung &amp; Compliance</h2>
<p>Die Software wurde von Grund auf nach dem Prinzip „Compliance by Design“ entwickelt, um die strengen globalen Vorgaben für Medizinprodukte zu erfüllen:</p>
<ul>
    <li><strong>Fälschungssichere MDR/FDA-Audit-Logs:</strong> Jeder Verarbeitungsschritt, jede Optimierung und jeder gewählte Hersteller-Filter (z.&nbsp;B. Siemens Deep Resolve, Philips SmartSpeed, GE AIR Recon DL, Canon AiCE, Fujifilm REiLI) wird vollautomatisch, kryptografisch verifiziert und absolut fälschungssicher protokolliert. Dies garantiert lückenlose Nachvollziehbarkeit bei Audits.</li>
    <li><strong>Datenschutz nach Weltstandard:</strong> Das System gewährleistet die strikte Einhaltung der europäischen Datenschutz-Grundverordnung (DSGVO) sowie des US-amerikanischen HIPAA-Gesetzes zur rechtssicheren Verarbeitung sensibler Patientendaten.</li>
</ul>

<div class="highlight-box">
    <h3>4. Social Impact: Spenden an Krankenhäuser</h3>
    <p>
        Technologie darf nicht nur Profite generieren — sie muss Menschen zugutekommen. Ein zentraler Bestandteil unseres Unternehmenskonzepts ist das <strong>Hospital Donation Program</strong>: Ein definierter Anteil jeder Sponsoring-Einnahme fließt direkt in die Ausstattung unterversorgter Kliniken mit Zugang zur AI-Denoising-Pipeline.
    </p>
    <p>
        Universitätskliniken und ländliche Krankenhäuser erhalten so kostenfreien oder stark subventionierten Zugang zu modernster Bildoptimierung — unabhängig von ihrem Budget. Damit verkürzen wir Diagnosezeiten, verbessern Bildqualität und entlasten medizinisches Fachpersonal in strukturschwachen Regionen.
    </p>
</div>

<div class="sponsor-box">
    <h3>5. Sponsoring &amp; Strategische Partnerschaften</h3>
    <p>Wir suchen Partner aus Medizintechnik, Radiologie und Venture Capital für die CE/MDR- und FDA-Zulassung:</p>
    <ul>
        <li><strong>Klinische Pilotstudien</strong> mit Chefärzten und Radiologen in Universitätskliniken</li>
        <li><strong>Technologie-Integration</strong> mit Siemens, Philips, GE, Canon und Fujifilm</li>
        <li><strong>Regulatorische Begleitung</strong> für MDR Klasse IIa und FDA 510(k)</li>
    </ul>
</div>

<div class="contact-card">
    <h3>Kontakt</h3>
    <table>
        <tr>
            <td class="label">Initiatorin:</td>
            <td>Inanna Roesner</td>
        </tr>
        <tr>
            <td class="label">E-Mail:</td>
            <td>inannaroesner07@gmail.com</td>
        </tr>
        <tr>
            <td class="label">Projekt:</td>
            <td>Medizintechnik-SaaS 2038 — Universal AI-Denoise-Engine</td>
        </tr>
        <tr>
            <td class="label">GitHub:</td>
            <td>github.com/Haneynanny/med_denoise_saas</td>
        </tr>
    </table>
</div>

<p class="ruo-notice">
    <strong>Research Use Only (RUO):</strong> Dieses System ist nicht für die klinische Diagnostik an echten Patienten zugelassen, solange die finale CE/MDR- oder FDA-Zertifizierung aussteht.
</p>

</body>
</html>
"""


def generate_pdf_weasyprint(html: str, pdf_path: Path, base_url: str) -> bool:
    """PDF via WeasyPrint (beste Qualität, benötigt GTK unter Windows)."""
    try:
        from weasyprint import HTML

        HTML(string=html, base_url=base_url).write_pdf(pdf_path)
        return True
    except OSError:
        return False


def generate_pdf_playwright(html_path: Path, pdf_path: Path) -> None:
    """Fallback: Chromium headless — zuverlässig unter Windows."""
    from playwright.sync_api import sync_playwright

    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()
        page.goto(html_path.as_uri())
        page.pdf(
            path=str(pdf_path),
            format="A4",
            margin={"top": "20mm", "right": "15mm", "bottom": "20mm", "left": "15mm"},
            print_background=True,
        )
        browser.close()


def generate_pdf_xhtml2pdf(html: str, pdf_path: Path) -> None:
    """Fallback: reines Python (vereinfachtes CSS ohne @page)."""
    from xhtml2pdf import pisa

    # xhtml2pdf unterstützt kein @page — vereinfachtes CSS
    simple_html = html.replace(
        "@page {\n    size: A4;\n    margin: 20mm 15mm;\n    background-color: #ffffff;\n    @bottom-right {\n        content: \"Seite \" counter(page);\n        font-family: 'Arial', sans-serif;\n        font-size: 9pt;\n        color: #718096;\n    }\n    @bottom-left {\n        content: \"Inanna Roesner — Medizintechnik-SaaS\";\n        font-family: 'Arial', sans-serif;\n        font-size: 9pt;\n        color: #718096;\n    }\n}",
        "",
    )

    with pdf_path.open("wb") as out_file:
        status = pisa.CreatePDF(simple_html, dest=out_file, encoding="utf-8")
    if status.err:
        raise RuntimeError(f"xhtml2pdf Fehlercode: {status.err}")


def main() -> None:
    root = Path(__file__).resolve().parent.parent
    docs_dir = root / "docs"
    docs_dir.mkdir(exist_ok=True)

    html_path = docs_dir / "Unternehmenskonzept_Sponsoring_Expose.html"
    pdf_path = docs_dir / "Unternehmenskonzept_Sponsoring_Expose.pdf"

    html_path.write_text(HTML_CONTENT, encoding="utf-8")
    print(f"HTML gespeichert: {html_path}")

    if generate_pdf_weasyprint(HTML_CONTENT, pdf_path, str(docs_dir)):
        print(f"PDF erstellt (WeasyPrint): {pdf_path}")
        return

    print("WeasyPrint nicht verfügbar — Fallback: Playwright/Chromium …")
    try:
        generate_pdf_playwright(html_path, pdf_path)
        print(f"PDF erstellt (Playwright): {pdf_path}")
        return
    except Exception as playwright_err:
        print(f"Playwright fehlgeschlagen ({playwright_err}) — Fallback: xhtml2pdf …")

    try:
        generate_pdf_xhtml2pdf(HTML_CONTENT, pdf_path)
    except ImportError as exc:
        raise SystemExit(
            "Bitte installieren: pip install playwright && playwright install chromium\n"
            "Oder: pip install xhtml2pdf"
        ) from exc

    print(f"PDF erstellt (xhtml2pdf): {pdf_path}")


if __name__ == "__main__":
    main()
