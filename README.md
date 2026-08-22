# Demo App for gtk_rs

> [!NOTE]
> This project is intended for testing and educational purposes only.

Repository template for rapidly bootstrapping **gtk_rs** application.

### Project Structure

```text
├── .cargo/config.toml
├── data
│   ├── resources
│   │   ├── meson.build
│   │   ├── resources.gresource.xml
│   │   └── window.ui
│   ├── meson.build
│   └── org.gtk_rs.DemoApp.gschema.xml
├── src
│   ├── window
│   │   ├── imp.rs
│   │   └── mod.rs
│   ├── meson.build
│   └── main.rs
├── meson.build
├── meson.options
├── README.md
├── Cargo.toml
└── Makefile.toml
```

### Running the Application

Execute the following command to start the application flow:

```bash
cargo make my_flow
```
> [!NOTE]
> Ensure **cargo-make** is installed on your system.