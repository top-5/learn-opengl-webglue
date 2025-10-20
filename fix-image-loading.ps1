# Fix WASM Image Loading Script
# Applies conditional compilation fix to all examples using image::open()

$files = @(
    "src/_1_getting_started/_5_1_transformations.rs",
    "src/_1_getting_started/_6_1_coordinate_systems.rs",
    "src/_1_getting_started/_6_2_coordinate_systems_depth.rs",
    "src/_1_getting_started/_6_3_coordinate_systems_multiple.rs",
    "src/_1_getting_started/_7_1_camera_circle.rs",
    "src/_1_getting_started/_7_2_camera_keyboard_dt.rs",
    "src/_1_getting_started/_7_3_camera_mouse_zoom.rs",
    "src/_1_getting_started/_7_4_camera_class.rs",
    "src/_4_advanced_opengl/_3_1_blending_discard.rs",
    "src/_4_advanced_opengl/_3_2_blending_sorted.rs",
    "src/_4_advanced_opengl/_6_1_cubemaps_skybox.rs",
    "src/_4_advanced_opengl/_6_2_cubemaps_environment_mapping.rs",
    "src/_5_advanced_lighting/_2_gamma_correction.rs",
    "src/_5_advanced_lighting/_6_hdr.rs",
    "src/_7_in_practice/_1_debugging.rs"
)

$fixedCount = 0
$skippedCount = 0

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Fix WASM Image Loading" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

foreach ($file in $files) {
    $fullPath = "c:\build\webglue\external\learn-opengl-rs\$file"
    
    if (-not (Test-Path $fullPath)) {
        Write-Host "✗ Not found: $file" -ForegroundColor Red
        $skippedCount++
        continue
    }
    
    $content = Get-Content $fullPath -Raw
    
    # Skip if already has WASM fix
    if ($content -match '#\[cfg\(target_arch = "wasm32"\)\]') {
        Write-Host "⊘ Already fixed: $file" -ForegroundColor Gray
        $skippedCount++
        continue
    }
    
    Write-Host "Fixing: $file" -ForegroundColor Yellow
    
    # Pattern 1: Single image::open() call
    $pattern1 = '(\s+)(let img = image::open\(&Path::new\("([^"]+)"\)\)\.expect\("([^"]+)"\);)'
    $replacement1 = '$1#[cfg(not(target_arch = "wasm32"))]' + "`n" + '$1let img = image::open(&Path::new("$3")).expect("$4");' + "`n" + '$1#[cfg(target_arch = "wasm32")]' + "`n" + '$1let img = {' + "`n" + '$1    use gl::resources::load_bytes_sync;' + "`n" + '$1    let bytes = load_bytes_sync("$3").expect("$4");' + "`n" + '$1    image::load_from_memory(&bytes).expect("Failed to decode texture")' + "`n" + '$1};'
    
    $newContent = $content -replace $pattern1, $replacement1
    
    if ($newContent -ne $content) {
        Set-Content -Path $fullPath -Value $newContent -NoNewline
        Write-Host "  ✓ Applied fix" -ForegroundColor Green
        $fixedCount++
    } else {
        Write-Host "  ⊘ No changes needed" -ForegroundColor Gray
        $skippedCount++
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Fixed: $fixedCount files" -ForegroundColor Green
Write-Host "Skipped: $skippedCount files" -ForegroundColor Gray
Write-Host ""
