use nannou::prelude::*;

fn main() {
  nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
  let draw = app.draw();
  draw.background().color(BLUE);
  draw.to_frame(app, &frame).unwrap();

  println!("Path {:#?}", capture_directory(app));

  let file_path = captured_frame_path(app, &frame);
  app.main_window().capture_frame(file_path)
}

/*
https://github.com/nannou-org/nannou/issues/758#issuecomment-866477840
*/

fn capture_directory(app: &App) -> std::path::PathBuf {
  app
    .project_path()
    .expect("could not locate project_path")
    .join(app.exe_name().unwrap())
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
  app
    .project_path()
    .expect("failed to locate `project_path`")
    .join(app.exe_name().unwrap())
    .join(format!("{:03}", frame.nth()))
    .with_extension("png")
}
