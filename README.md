# Nebula
An Open Modern Operating System

⚠️
__Please note that the project is in the very early stages.__
__It is not ready for neither evaluation nor testing.__
__Features listed below are basically goals and most probably not yet implemented.__
⚠️

Features:
* Ecosystem written from scratch
  * 📡 Nebula Fabric - a system bus, based on protocol buffers and zeromq
  * 🌌 Nebula Ether - a system daemon process similar to systemd
  * 📦 [Nebula Package Keeper](https://github.com/skyne98/npk) - fast secure package manager with sandboxing
  * 🖥️ Nebula Compositor - low-latency Wayland-based window manager/compositor
  * __... more components planned in the future__
* Security
  * ⚙️ Most recent Linux kernel updates
  * 🔐 Application sandboxing system similar to Android™️
  * 🔑 Centralized permission management
* Speed
  * 🦀 Core components written in Rust
  * 🚀 Heavily parallelized architecture
  * 💻 Made with modern hardware in mind
* Language-agnostic standard library
  * Based on Nebula Fabric
  * Available in C#, Rust and TypeScript
  * Unified access point to core Nebula services
  * Unified graphics APIs based on GFX-hal
* Centralized infrastructure
  * Based on services, maintained by Ether
  * Registry-like structured system-wide configuration
  * System-wide user permission management

Inspirations:
* Linux
  * Open-source ideology with optional commercialization
  * Wayland compositor protocol
  * Package management
* Windows
  * Service-based system architecture
  * Simple user-friendly configuration
  * Casual user oriented experience
  * Overall UI speed and latency
  * UWP performance guidelines
  * UWP touch/mouse/pen inputs
  * .NET Core standard library
* MacOS
  * Smooth UI experience
  * Unified stylized look
  * Touchpad experience
* HaikuOS
  * Heavily multithreaded architecture
  * Services "attached" to processes
* Android
  * Application sandboxing
  * More centralized system architecture
  * Mobile, touch friendly
