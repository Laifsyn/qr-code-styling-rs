//! Generate sample images for README documentation.

use qr_code_styling::config::{
    BackgroundOptions, Color, CornersDotOptions, CornersSquareOptions, DotsOptions, Gradient,
    ImageOptions,
};
use qr_code_styling::plugins::border::{BorderPlugin, Position, QRBorderOptions};
use qr_code_styling::types::{CornerDotType, CornerSquareType, DotType, OutputFormat, ShapeType};
use qr_code_styling::{QRCodeStyling, QRCodeStylingBuilder};
use std::path::{Path, PathBuf};

fn main() -> qr_code_styling::error::Result<()> {
    let assets = "assets";
    std::fs::create_dir_all(assets)?;

    // 1. Basic - simple default QR
    println!("Generating basic sample...");
    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .build()?;
    qr.save(&format!("{}/basic.png", assets), OutputFormat::Png)?;

    // 2. Rounded dots
    println!("Generating rounded dots sample...");
    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .dots_options(
            DotsOptions::new(DotType::Rounded).with_color(Color::from_hex("#4A90D9").unwrap()),
        )
        .corners_square_options(
            CornersSquareOptions::new(CornerSquareType::ExtraRounded)
                .with_color(Color::from_hex("#4A90D9").unwrap()),
        )
        .corners_dot_options(
            CornersDotOptions::new(CornerDotType::Dot)
                .with_color(Color::from_hex("#4A90D9").unwrap()),
        )
        .background_options(
            BackgroundOptions::default().with_color(Color::from_hex("#FFFFFF").unwrap()),
        )
        .build()?;
    qr.save(&format!("{}/rounded.png", assets), OutputFormat::Png)?;

    // 3. Dots style
    println!("Generating dots style sample...");
    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .dots_options(
            DotsOptions::new(DotType::Dots).with_color(Color::from_hex("#E74C3C").unwrap()),
        )
        .corners_square_options(
            CornersSquareOptions::new(CornerSquareType::Dot)
                .with_color(Color::from_hex("#C0392B").unwrap()),
        )
        .corners_dot_options(
            CornersDotOptions::new(CornerDotType::Dot)
                .with_color(Color::from_hex("#C0392B").unwrap()),
        )
        .background_options(
            BackgroundOptions::default().with_color(Color::from_hex("#FFFFFF").unwrap()),
        )
        .build()?;
    qr.save(&format!("{}/dots.png", assets), OutputFormat::Png)?;

    // 4. Classy rounded
    println!("Generating classy rounded sample...");
    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .dots_options(
            DotsOptions::new(DotType::ClassyRounded)
                .with_color(Color::from_hex("#2C3E50").unwrap()),
        )
        .corners_square_options(
            CornersSquareOptions::new(CornerSquareType::ExtraRounded)
                .with_color(Color::from_hex("#2C3E50").unwrap()),
        )
        .corners_dot_options(
            CornersDotOptions::new(CornerDotType::Dot)
                .with_color(Color::from_hex("#2C3E50").unwrap()),
        )
        .background_options(
            BackgroundOptions::default().with_color(Color::from_hex("#FFFFFF").unwrap()),
        )
        .build()?;
    qr.save(&format!("{}/classy_rounded.png", assets), OutputFormat::Png)?;

    // 5. With gradient
    println!("Generating gradient sample...");
    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .dots_options(
            DotsOptions::new(DotType::Rounded).with_gradient(Gradient::simple_linear(
                Color::from_hex("#8E2DE2").unwrap(),
                Color::from_hex("#4A00E0").unwrap(),
            )),
        )
        .corners_square_options(
            CornersSquareOptions::new(CornerSquareType::ExtraRounded)
                .with_color(Color::from_hex("#8E2DE2").unwrap()),
        )
        .corners_dot_options(
            CornersDotOptions::new(CornerDotType::Dot)
                .with_color(Color::from_hex("#4A00E0").unwrap()),
        )
        .background_options(
            BackgroundOptions::default().with_color(Color::from_hex("#FFFFFF").unwrap()),
        )
        .build()?;
    qr.save(&format!("{}/gradient.png", assets), OutputFormat::Png)?;

    // 6. With logo
    println!("Generating logo sample...");
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let logo_path = PathBuf::from(manifest_dir).join("examples/logo.png");
    let logo_bytes = std::fs::read(&logo_path)?;

    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .image(logo_bytes.clone())
        .image_options(
            ImageOptions::default()
                .with_image_size(0.4)
                .with_margin(5)
                .with_hide_background_dots(true),
        )
        .dots_options(
            DotsOptions::new(DotType::Rounded).with_color(Color::from_hex("#1877F2").unwrap()),
        )
        .corners_square_options(
            CornersSquareOptions::new(CornerSquareType::ExtraRounded)
                .with_color(Color::from_hex("#1877F2").unwrap()),
        )
        .corners_dot_options(
            CornersDotOptions::new(CornerDotType::Dot)
                .with_color(Color::from_hex("#1877F2").unwrap()),
        )
        .background_options(
            BackgroundOptions::default().with_color(Color::from_hex("#FFFFFF").unwrap()),
        )
        .build()?;
    qr.save(&format!("{}/with_logo.png", assets), OutputFormat::Png)?;

    // 7. Circle shape
    println!("Generating circle shape sample...");
    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .shape(ShapeType::Circle)
        .dots_options(
            DotsOptions::new(DotType::Rounded).with_color(Color::from_hex("#27AE60").unwrap()),
        )
        .corners_square_options(
            CornersSquareOptions::new(CornerSquareType::ExtraRounded)
                .with_color(Color::from_hex("#27AE60").unwrap()),
        )
        .corners_dot_options(
            CornersDotOptions::new(CornerDotType::Dot)
                .with_color(Color::from_hex("#27AE60").unwrap()),
        )
        .background_options(
            BackgroundOptions::default().with_color(Color::from_hex("#FFFFFF").unwrap()),
        )
        .build()?;
    qr.save(&format!("{}/circle.png", assets), OutputFormat::Png)?;

    // 8. With border
    println!("Generating border sample...");
    let qr = QRCodeStyling::builder()
        .data("https://github.com/nazrdogan/qr-code-styling-rs")
        .size(300)
        .margin(50)
        .shape(ShapeType::Circle)
        .image(logo_bytes)
        .image_options(
            ImageOptions::default()
                .with_image_size(0.3)
                .with_margin(5)
                .with_hide_background_dots(true),
        )
        .dots_options(
            DotsOptions::new(DotType::Rounded).with_color(Color::from_hex("#E74C3C").unwrap()),
        )
        .corners_square_options(
            CornersSquareOptions::new(CornerSquareType::ExtraRounded)
                .with_color(Color::from_hex("#E74C3C").unwrap()),
        )
        .corners_dot_options(
            CornersDotOptions::new(CornerDotType::Dot)
                .with_color(Color::from_hex("#E74C3C").unwrap()),
        )
        .build()?;

    let svg = qr.render_svg()?;

    let text_style =
        "font-size: 18px; font-family: Arial, sans-serif; fill: #FFFFFF; font-weight: bold;";

    let border_options = QRBorderOptions::new(35.0, "#E74C3C")
        .with_round(1.0)
        .with_styled_text(Position::Top, "SCAN ME", text_style)
        .with_styled_text(Position::Bottom, "qr-code-styling", text_style);

    let bordered_svg = BorderPlugin::new(border_options).apply(&svg, 300, 300);

    // Convert bordered SVG to PNG via saving as SVG then rendering
    std::fs::write(&format!("{}/with_border.svg", assets), &bordered_svg)?;

    println!("\nAll samples generated in assets/!");

    // scaffold a set of samples based on different categories.
    const QR_SIZE: u32 = 300;

    let base_builder = QRCodeStyling::builder().data(SAMPLE_DATA).size(QR_SIZE);
    let base_background =
        BackgroundOptions::default().with_color(Color::from_hex("#333333").unwrap());
    let root: &Path = assets.as_ref();

    // 9. Gradient Samples
    let gradient_samples = samples_background_gradients(base_builder, base_background)?;
    let gradients_path = root.join("background_gradients");
    save(gradient_samples, &gradients_path)?;

    // 10. Dot Samples

    Ok(())
}

struct Sample {
    /// Name of the sample (used for filename)
    name: &'static str,
    /// QR code configuration for this sample
    style: QRCodeStyling,
}

/// Receives a list of samples and saves them to the specified root path.
///
/// Each sample's name is used to generate the filename (e.g., "{root_path}/linear_gradient.png").
fn save(samples: Vec<Sample>, root_path: &Path) -> qr_code_styling::error::Result<()> {
    std::fs::create_dir_all(root_path)?;
    for sample in samples {
        let file_path = root_path.join(format!("{}.png", sample.name));
        sample.style.save(&file_path, OutputFormat::Png)?;
    }
    Ok(())
}

/// Use the Repository's URL as the QR's data.
const SAMPLE_DATA: &str = "https://github.com/nazrdogan/qr-code-styling-rs";

/// helper macro to return stringification of a variable along with its value
/// # Example:
///
/// ```
/// let my_var = 42;
/// let (name, value) = into_tuple!(my_var);
/// assert_eq!(name, "my_var");
/// assert_eq!(value, 42);
/// ```
macro_rules! into_tuple {
    ($var:ident) => {
        (stringify!($var), $var)
    };
}

/// Returns a list of sample QR code configurations with different background [gradients types]().
///
/// Receives a basic configurations to act as the base styling for generated samples.
fn samples_background_gradients(
    base_styling: QRCodeStylingBuilder,
    base_background: BackgroundOptions,
) -> qr_code_styling::error::Result<Vec<Sample>> {
    use qr_code_styling::ColorStop;
    let stop_0 = ColorStop::new(0.0, Color::from_hex("#FFEE88").unwrap());
    let stop_1 = ColorStop::new(0.5, Color::from_hex("#AA1155").unwrap());
    let stop_2 = ColorStop::new(1.0, Color::from_hex("#00CC99").unwrap());

    let linear_gradient = Gradient::linear(vec![stop_0.clone(), stop_1.clone(), stop_2.clone()]);
    let radial_gradient = Gradient::radial(vec![stop_0.clone(), stop_1.clone(), stop_2.clone()]);
    let gradient_options = vec![into_tuple!(linear_gradient), into_tuple!(radial_gradient)];

    let mut styles = vec![];
    for (name, gradient) in gradient_options {
        let style = base_styling
            .clone()
            .background_options(base_background.clone().with_gradient(gradient))
            .build()?;

        styles.push(Sample { name, style });
    }

    Ok(styles)
}

#[test]
fn test_into_tuple() {
    let my_var = 42;
    let (name, value): (&str, i8) = into_tuple!(my_var);
    assert_eq!(name, "my_var");
    assert_eq!(value, 42);
}
