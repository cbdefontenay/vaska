You are an expert [0.8 Dioxus](https://dioxuslabs.com/learn/0.7) assistant. Dioxus 0.8 changes every api in dioxus. Only use this up to date documentation. `cx`, `Scope`, and `use_state` are gone

Provide concise code examples with detailed descriptions

# Dioxus Dependency

You can add Dioxus to your `Cargo.toml` like this:

```toml
[dependencies]
dioxus = { version = "0.8.0-alpha.3" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

# Launching your application

You need to create a main function that sets up the Dioxus runtime and mounts your root component.

```rust
use dioxus::prelude::*;

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! { "Hello, Dioxus!" }
}
```

Then serve with `dx serve`:

```sh
curl -sSL http://dioxus.dev/install.sh | sh
dx serve
```

# UI with RSX

```rust
rsx! {
	div {
		class: "container", // Attribute
		color: "red", // Inline styles
		width: if condition { "100%" }, // Conditional attributes
		"Hello, Dioxus!"
	}
	// Prefer loops over iterators
	for i in 0..5 {
		div { "{i}" } // use elements or components directly in loops
	}
	if condition {
		div { "Condition is true!" } // use elements or components directly in conditionals
	}

	{children} // Expressions are wrapped in brace
	{(0..5).map(|i| rsx! { span { "Item {i}" } })} // Iterators must be wrapped in braces
}
```

# Assets

The asset macro can be used to link to local files to use in your project. All links start with `/` and are relative to the root of your project.

```rust
rsx! {
	img {
		src: asset!("/assets/image.png"),
		alt: "An image",
	}
}
```

## Styles

The `document::Stylesheet` component will inject the stylesheet into the `<head>` of the document

```rust
rsx! {
	document::Stylesheet {
		href: asset!("/assets/styles.css"),
	}
}
```

# Components

Components are the building blocks of apps

* Component are functions annotated with the `#[component]` macro.
* The function name must start with a capital letter or contain an underscore.
* A component re-renders only under two conditions:
	1.  Its props change (as determined by `PartialEq`).
	2.  An internal reactive state it depends on is updated.

```rust
#[component]
fn Input(mut value: Signal<String>) -> Element {
	rsx! {
		input {
            value,
			oninput: move |e| {
				*value.write() = e.value();
			},
			onkeydown: move |e| {
				if e.key() == Key::Enter {
					value.write().clear();
				}
			},
		}
	}
}
```

Each component accepts function arguments (props)

* Props must be owned values, not references. Use `String` and `Vec<T>` instead of `&str` or `&[T]`.
* Props must implement `PartialEq` and `Clone`.
* To make props reactive and copy, you can wrap the type in `ReadOnlySignal`. Any reactive state like memos and resources that read `ReadOnlySignal` props will automatically re-run when the prop changes.

# State

A signal is a wrapper around a value that automatically tracks where it's read and written. Changing a signal's value causes code that relies on the signal to rerun.

## Local State

The `use_signal` hook creates state that is local to a single component. You can call the signal like a function (e.g. `my_signal()`) to clone the value, or use `.read()` to get a reference. `.write()` gets a mutable reference to the value.

Use `use_memo` to create a memoized value that recalculates when its dependencies change. Memos are useful for expensive calculations that you don't want to repeat unnecessarily.

```rust
#[component]
fn Counter() -> Element {
	let mut count = use_signal(|| 0);
	let mut doubled = use_memo(move || count() * 2); // doubled will re-run when count changes because it reads the signal

	rsx! {
		h1 { "Count: {count}" } // Counter will re-render when count changes because it reads the signal
		h2 { "Doubled: {doubled}" }
		button {
			onclick: move |_| *count.write() += 1, // Writing to the signal rerenders Counter
			"Increment"
		}
		button {
			onclick: move |_| count.with_mut(|count| *count += 1), // use with_mut to mutate the signal
			"Increment with with_mut"
		}
	}
}
```

## Context API

The Context API allows you to share state down the component tree. A parent provides the state using `use_context_provider`, and any child can access it with `use_context`

```rust
#[component]
fn App() -> Element {
	let mut theme = use_signal(|| "light".to_string());
	use_context_provider(|| theme); // Provide a type to children
	rsx! { Child {} }
}

#[component]
fn Child() -> Element {
	let theme = use_context::<Signal<String>>(); // Consume the same type
	rsx! {
		div {
			"Current theme: {theme}"
		}
	}
}
```

# Async

For state that depends on an asynchronous operation (like a network request), Dioxus provides a hook called `use_resource`. This hook manages the lifecycle of the async task and provides the result to your component.

* The `use_resource` hook takes an `async` closure. It re-runs this closure whenever any signals it depends on (reads) are updated
* The `Resource` object returned can be in several states when read:
1. `None` if the resource is still loading
2. `Some(value)` if the resource has successfully loaded

```rust
let mut dog = use_resource(move || async move {
	// api request
});

match dog() {
	Some(dog_info) => rsx! { Dog { dog_info } },
	None => rsx! { "Loading..." },
}
```

# Routing

All possible routes are defined in a single Rust `enum` that derives `Routable`. Each variant represents a route and is annotated with `#[route("/path")]`. Dynamic Segments can capture parts of the URL path as parameters by using `:name` in the route string. These become fields in the enum variant.

The `Router<Route> {}` component is the entry point that manages rendering the correct component for the current URL.

You can use the `#[layout(NavBar)]` to create a layout shared between pages and place an `Outlet<Route> {}` inside your layout component. The child routes will be rendered in the outlet.

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
	#[layout(NavBar)] // This will use NavBar as the layout for all routes
		#[route("/")]
		Home {},
		#[route("/blog/:id")] // Dynamic segment
		BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
	rsx! {
		a { href: "/", "Home" }
		Outlet<Route> {} // Renders Home or BlogPost
	}
}

#[component]
fn App() -> Element {
	rsx! { Router::<Route> {} }
}
```

```toml
dioxus = { version = "0.8.0-alpha.0", features = ["router"] }
```

# Fullstack

Fullstack enables server rendering and ipc calls. It uses Cargo features (`server` and a client feature like `web`) to split the code into a server and client binaries.

```toml
dioxus = { version = "0.8.0-alpha.0", features = ["fullstack"] }
```

## Server Functions

Use the `#[post]` / `#[get]` macros to define an `async` function that will only run on the server. On the server, this macro generates an API endpoint. On the client, it generates a function that makes an HTTP request to that endpoint.

```rust
#[post("/api/double/:path/&query")]
async fn double_server(number: i32, path: String, query: i32) -> Result<i32, ServerFnError> {
	tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	Ok(number * 2)
}
```

## Hydration

Hydration is the process of making a server-rendered HTML page interactive on the client. The server sends the initial HTML, and then the client-side runs, attaches event listeners, and takes control of future rendering.

### Errors
The initial UI rendered by the component on the client must be identical to the UI rendered on the server.

* Use the `use_server_future` hook instead of `use_resource`. It runs the future on the server, serializes the result, and sends it to the client, ensuring the client has the data immediately for its first render.
* Any code that relies on browser-specific APIs (like accessing `localStorage`) must be run *after* hydration. Place this code inside a `use_effect` hook.


<SYSTEM>This is the developer documentation for Dioxus from /learn/0.7/tutorial/data_fetching.</SYSTEM>
# Fetching Data

## Adding Dependencies

Dioxus does not provide any built-in utilities for fetching data. Crates like [dioxus-query](https://github.com/marc2332/dioxus-query) exist, but for this tutorial we'll implement data-fetching from scratch.

First, we need to add two new dependencies to our app: [serde](https://crates.io/crates/serde) and [reqwest](https://crates.io/crates/reqwest).

* Reqwest provides an HTTP client for fetching.
* Serde will let us derive a JSON Deserializer to decode the response.

In a new terminal window, add these crates to your app with `cargo add`.

````bash
cargo add reqwest --features json
cargo add serde --features derive
````

## Defining a Response Type

We'll be using the amazing [dog.ceo/dog-api](https://dog.ceo/dog-api/) to fetch images of dogs for *HotDog*. Fortunately, the API response is quite simple to deserialize. Let's create a new Rust struct that matches the format of the API and derive `Deserialize` for it.

The Dog API docs outline a sample API response:

````json
{
    "message": "https://images.dog.ceo/breeds/leonberg/n02111129_974.jpg",
    "status": "success"
}
````

Our Rust struct needs to match that format, though for now we'll only include the "message" field.

````rs@guide_data_fetching.rs
#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}
````

## Using `reqwest` and `async`

Dioxus has great support for asynchronous Rust. We can simply convert our `onclick` handler to be `async` and then set the `img_src` after the future has resolved.

![Dog Fetching](/assets/06_docs/fetch-dog.mp4)

The changes to our code are quite simple - just add the `reqwest::get` call and then call `.set()` on `img_src` with the result.

````rs@guide_data_fetching.rs
#[component]
fn DogView() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    let save = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message);
    };

    // ..

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            // ..
            button { onclick: save, id: "save", "save!" }
        }
    }
}
````

Dioxus automatically calls `dioxus::spawn` on asynchronous closures. You can also use `dioxus::spawn` to perform async work *without* async closures - just call `dioxus::spawn()` on any async block. Futures spawned with `dioxus::spawn` automatically run on the current async executor and are dropped automatically.

````rs@guide_data_fetching.rs
rsx! {
    button {
        onclick: move |_| {
            spawn(async move {
                // do some async work...
            });
        }
    }
}
````

The futures passed to `dioxus::spawn` can not borrow data from outside the async block. Data that is `Copy` *can* be captured by async blocks, but all other data must be *moved*, usually by calling `.clone()`.

## Data Fetching with `use_resource`

Eventually, using bare `async` calls might lead to race conditions and weird state bugs. For example, if the user clicks the *fetch* button too quickly, then two requests will be made in parallel. If the request is updating data somewhere else, the wrong request might finish early and causes a race condition.

In Dioxus, *Resources* are pieces of state whose value is dependent on the completion of some asynchronous work. The `use_resource` hook provides a `Resource` object with helpful methods to start, stop, pause, and modify the asynchronous state.

Let's change our component to use a resource instead:

````rs@guide_data_fetching.rs
#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: move |_| img_src.restart(), id: "save", "save!" }
        }
    }
}
````

Resources are very powerful: they integrate with Suspense, Streaming HTML, reactivity, and more.

The details of the `Resource` API are not terribly important right now, but you'll be using Resources frequently in larger apps, so it's a good idea to [read the docs](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html).
# Fetching Data

## Adding Dependencies

Dioxus does not provide any built-in utilities for fetching data. Crates like [dioxus-query](https://github.com/marc2332/dioxus-query) exist, but for this tutorial we'll implement data-fetching from scratch.

First, we need to add two new dependencies to our app: [serde](https://crates.io/crates/serde) and [reqwest](https://crates.io/crates/reqwest).

* Reqwest provides an HTTP client for fetching.
* Serde will let us derive a JSON Deserializer to decode the response.

In a new terminal window, add these crates to your app with `cargo add`.

````bash
cargo add reqwest --features json
cargo add serde --features derive
````

## Defining a Response Type

We'll be using the amazing [dog.ceo/dog-api](https://dog.ceo/dog-api/) to fetch images of dogs for *HotDog*. Fortunately, the API response is quite simple to deserialize. Let's create a new Rust struct that matches the format of the API and derive `Deserialize` for it.

The Dog API docs outline a sample API response:

````json
{
    "message": "https://images.dog.ceo/breeds/leonberg/n02111129_974.jpg",
    "status": "success"
}
````

Our Rust struct needs to match that format, though for now we'll only include the "message" field.

````rs@guide_data_fetching.rs
#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}
````

## Using `reqwest` and `async`

Dioxus has great support for asynchronous Rust. We can simply convert our `onclick` handler to be `async` and then set the `img_src` after the future has resolved.

![Dog Fetching](/assets/06_docs/fetch-dog.mp4)

The changes to our code are quite simple - just add the `reqwest::get` call and then call `.set()` on `img_src` with the result.

````rs@guide_data_fetching.rs
#[component]
fn DogView() -> Element {
    let mut img_src = use_signal(|| "".to_string());

    let save = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();

        img_src.set(response.message);
    };

    // ..

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            // ..
            button { onclick: save, id: "save", "save!" }
        }
    }
}
````

Dioxus automatically calls `dioxus::spawn` on asynchronous closures. You can also use `dioxus::spawn` to perform async work *without* async closures - just call `dioxus::spawn()` on any async block. Futures spawned with `dioxus::spawn` automatically run on the current async executor and are dropped automatically.

````rs@guide_data_fetching.rs
rsx! {
    button {
        onclick: move |_| {
            spawn(async move {
                // do some async work...
            });
        }
    }
}
````

The futures passed to `dioxus::spawn` can not borrow data from outside the async block. Data that is `Copy` *can* be captured by async blocks, but all other data must be *moved*, usually by calling `.clone()`.

## Data Fetching with `use_resource`

Eventually, using bare `async` calls might lead to race conditions and weird state bugs. For example, if the user clicks the *fetch* button too quickly, then two requests will be made in parallel. If the request is updating data somewhere else, the wrong request might finish early and causes a race condition.

In Dioxus, *Resources* are pieces of state whose value is dependent on the completion of some asynchronous work. The `use_resource` hook provides a `Resource` object with helpful methods to start, stop, pause, and modify the asynchronous state.

Let's change our component to use a resource instead:

````rs@guide_data_fetching.rs
#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: move |_| img_src.restart(), id: "save", "save!" }
        }
    }
}
````

Resources are very powerful: they integrate with Suspense, Streaming HTML, reactivity, and more.

The details of the `Resource` API are not terribly important right now, but you'll be using Resources frequently in larger apps, so it's a good idea to [read the docs](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html).

<SYSTEM>This is the developer documentation for Dioxus from /learn/0.7/guides/deploy/config.</SYSTEM>
### Bundling config

The `[bundle]` section of our Dioxus.toml can take a variety of options.

Here are the options, in the form of Rust structs.

````rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct BundleConfig {
    /// eg. com.dioxuslabs
    pub(crate) identifier: Option<String>,
    /// eg. DioxusLabs
    pub(crate) publisher: Option<String>,
    /// eg. assets/icon.png
    pub(crate) icon: Option<Vec<String>>,
    /// eg. Extra assets like "img.png"
    pub(crate) resources: Option<Vec<String>>,
    /// eg. DioxusLabs
    pub(crate) copyright: Option<String>,
    /// eg. "Social Media"
    pub(crate) category: Option<String>,
    /// eg. "A great social media app"
    pub(crate) short_description: Option<String>,
    /// eg. "A social media app that makes people love app development"
    pub(crate) long_description: Option<String>,
    /// eg. extra binaries (like tools) to include in the final app
    pub(crate) external_bin: Option<Vec<String>>,
    /// Additional debian-only settings (see below)
    pub(crate) deb: Option<DebianSettings>,
    /// Additional macos settings (see below)
    pub(crate) macos: Option<MacOsSettings>,
    /// Additional windows settings (see below)
    pub(crate) windows: Option<WindowsSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DebianSettings {
    // OS-specific settings:
    /// the list of debian dependencies.
    pub depends: Option<Vec<String>>,
    /// the list of dependencies the package provides.
    pub provides: Option<Vec<String>>,
    /// the list of package conflicts.
    pub conflicts: Option<Vec<String>>,
    /// the list of package replaces.
    pub replaces: Option<Vec<String>>,
    /// List of custom files to add to the deb package.
    /// Maps the path on the debian package to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Path to a custom desktop file Handlebars template.
    ///
    /// Available variables: `categories`, `comment` (optional), `exec`, `icon` and `name`.
    pub desktop_template: Option<PathBuf>,
    /// Define the section in Debian Control file. See : <https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections>
    pub section: Option<String>,
    /// Change the priority of the Debian Package. By default, it is set to `optional`.
    /// Recognized Priorities as of now are :  `required`, `important`, `standard`, `optional`, `extra`
    pub priority: Option<String>,
    /// Path of the uncompressed Changelog file, to be stored at /usr/share/doc/package-name/changelog.gz. See
    /// <https://www.debian.org/doc/debian-policy/ch-docs.html#changelog-files-and-release-notes>
    pub changelog: Option<PathBuf>,
    /// Path to script that will be executed before the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_install_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_install_script: Option<PathBuf>,
    /// Path to script that will be executed before the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_remove_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_remove_script: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct WixSettings {
    pub(crate) language: Vec<(String, Option<PathBuf>)>,
    pub(crate) template: Option<PathBuf>,
    pub(crate) fragment_paths: Vec<PathBuf>,
    pub(crate) component_group_refs: Vec<String>,
    pub(crate) component_refs: Vec<String>,
    pub(crate) feature_group_refs: Vec<String>,
    pub(crate) feature_refs: Vec<String>,
    pub(crate) merge_refs: Vec<String>,
    pub(crate) skip_webview_install: bool,
    pub(crate) license: Option<PathBuf>,
    pub(crate) enable_elevated_update_task: bool,
    pub(crate) banner_path: Option<PathBuf>,
    pub(crate) dialog_image_path: Option<PathBuf>,
    pub(crate) fips_compliant: bool,
    /// MSI installer version in the format `major.minor.patch.build` (build is optional).
    ///
    /// Because a valid version is required for MSI installer, it will be derived from [`PackageSettings::version`] if this field is not set.
    ///
    /// The first field is the major version and has a maximum value of 255. The second field is the minor version and has a maximum value of 255.
    /// The third and fourth fields have a maximum value of 65,535.
    ///
    /// See <https://learn.microsoft.com/en-us/windows/win32/msi/productversion> for more info.
    pub version: Option<String>,
    /// A GUID upgrade code for MSI installer. This code **_must stay the same across all of your updates_**,
    /// otherwise, Windows will treat your update as a different app and your users will have duplicate versions of your app.
    ///
    /// By default, tauri generates this code by generating a Uuid v5 using the string `<productName>.exe.app.x64` in the DNS namespace.
    /// You can use Tauri's CLI to generate and print this code for you by running `tauri inspect wix-upgrade-code`.
    ///
    /// It is recommended that you set this value in your tauri config file to avoid accidental changes in your upgrade code
    /// whenever you want to change your product name.
    pub upgrade_code: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct MacOsSettings {
    pub(crate) frameworks: Option<Vec<String>>,
    pub(crate) minimum_system_version: Option<String>,
    pub(crate) license: Option<String>,
    pub(crate) exception_domain: Option<String>,
    pub(crate) signing_identity: Option<String>,
    pub(crate) provider_short_name: Option<String>,
    pub(crate) entitlements: Option<String>,
    pub(crate) info_plist_path: Option<PathBuf>,
    /// List of custom files to add to the application bundle.
    /// Maps the path in the Contents directory in the app to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Preserve the hardened runtime version flag, see <https://developer.apple.com/documentation/security/hardened_runtime>
    ///
    /// Settings this to `false` is useful when using an ad-hoc signature, making it less strict.
    pub hardened_runtime: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WindowsSettings {
    pub(crate) digest_algorithm: Option<String>,
    pub(crate) certificate_thumbprint: Option<String>,
    pub(crate) timestamp_url: Option<String>,
    pub(crate) tsp: bool,
    pub(crate) wix: Option<WixSettings>,
    pub(crate) icon_path: Option<PathBuf>,
    pub(crate) webview_install_mode: WebviewInstallMode,
    pub(crate) webview_fixed_runtime_path: Option<PathBuf>,
    pub(crate) allow_downgrades: bool,
    pub(crate) nsis: Option<NsisSettings>,
    /// Specify a custom command to sign the binaries.
    /// This command needs to have a `%1` in it which is just a placeholder for the binary path,
    /// which we will detect and replace before calling the command.
    ///
    /// Example:
    /// ```text
    /// sign-cli --arg1 --arg2 %1
    /// ```
    ///
    /// By Default we use `signtool.exe` which can be found only on Windows so
    /// if you are on another platform and want to cross-compile and sign you will
    /// need to use another tool like `osslsigncode`.
    pub sign_command: Option<CustomSignCommandSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NsisSettings {
    pub(crate) template: Option<PathBuf>,
    pub(crate) license: Option<PathBuf>,
    pub(crate) header_image: Option<PathBuf>,
    pub(crate) sidebar_image: Option<PathBuf>,
    pub(crate) installer_icon: Option<PathBuf>,
    pub(crate) install_mode: NSISInstallerMode,
    pub(crate) languages: Option<Vec<String>>,
    pub(crate) custom_language_files: Option<HashMap<String, PathBuf>>,
    pub(crate) display_language_selector: bool,
    pub(crate) start_menu_folder: Option<String>,
    pub(crate) installer_hooks: Option<PathBuf>,
    /// Try to ensure that the WebView2 version is equal to or newer than this version,
    /// if the user's WebView2 is older than this version,
    /// the installer will try to trigger a WebView2 update.
    pub minimum_webview2_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum NSISInstallerMode {
    CurrentUser,
    PerMachine,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum WebviewInstallMode {
    Skip,
    DownloadBootstrapper { silent: bool },
    EmbedBootstrapper { silent: bool },
    OfflineInstaller { silent: bool },
    FixedRuntime { path: PathBuf },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSignCommandSettings {
    /// The command to run to sign the binary.
    pub cmd: String,
    /// The arguments to pass to the command.
    ///
    /// "%1" will be replaced with the path to the binary to be signed.
    pub args: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PackageType {
    /// "macos"
    MacOsBundle,
    /// "ios"
    IosBundle,
    /// "msi"
    WindowsMsi,
    /// "nsis"
    Nsis,
    /// "deb"
    Deb,
    /// "rpm"
    Rpm,
    /// "appimage"
    AppImage,
    /// "dmg"
    Dmg,
    /// "updater"
    Updater,
}
````
### Bundling config

The `[bundle]` section of our Dioxus.toml can take a variety of options.

Here are the options, in the form of Rust structs.

````rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct BundleConfig {
    /// eg. com.dioxuslabs
    pub(crate) identifier: Option<String>,
    /// eg. DioxusLabs
    pub(crate) publisher: Option<String>,
    /// eg. assets/icon.png
    pub(crate) icon: Option<Vec<String>>,
    /// eg. Extra assets like "img.png"
    pub(crate) resources: Option<Vec<String>>,
    /// eg. DioxusLabs
    pub(crate) copyright: Option<String>,
    /// eg. "Social Media"
    pub(crate) category: Option<String>,
    /// eg. "A great social media app"
    pub(crate) short_description: Option<String>,
    /// eg. "A social media app that makes people love app development"
    pub(crate) long_description: Option<String>,
    /// eg. extra binaries (like tools) to include in the final app
    pub(crate) external_bin: Option<Vec<String>>,
    /// Additional debian-only settings (see below)
    pub(crate) deb: Option<DebianSettings>,
    /// Additional macos settings (see below)
    pub(crate) macos: Option<MacOsSettings>,
    /// Additional windows settings (see below)
    pub(crate) windows: Option<WindowsSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DebianSettings {
    // OS-specific settings:
    /// the list of debian dependencies.
    pub depends: Option<Vec<String>>,
    /// the list of dependencies the package provides.
    pub provides: Option<Vec<String>>,
    /// the list of package conflicts.
    pub conflicts: Option<Vec<String>>,
    /// the list of package replaces.
    pub replaces: Option<Vec<String>>,
    /// List of custom files to add to the deb package.
    /// Maps the path on the debian package to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Path to a custom desktop file Handlebars template.
    ///
    /// Available variables: `categories`, `comment` (optional), `exec`, `icon` and `name`.
    pub desktop_template: Option<PathBuf>,
    /// Define the section in Debian Control file. See : <https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections>
    pub section: Option<String>,
    /// Change the priority of the Debian Package. By default, it is set to `optional`.
    /// Recognized Priorities as of now are :  `required`, `important`, `standard`, `optional`, `extra`
    pub priority: Option<String>,
    /// Path of the uncompressed Changelog file, to be stored at /usr/share/doc/package-name/changelog.gz. See
    /// <https://www.debian.org/doc/debian-policy/ch-docs.html#changelog-files-and-release-notes>
    pub changelog: Option<PathBuf>,
    /// Path to script that will be executed before the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_install_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_install_script: Option<PathBuf>,
    /// Path to script that will be executed before the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_remove_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_remove_script: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct WixSettings {
    pub(crate) language: Vec<(String, Option<PathBuf>)>,
    pub(crate) template: Option<PathBuf>,
    pub(crate) fragment_paths: Vec<PathBuf>,
    pub(crate) component_group_refs: Vec<String>,
    pub(crate) component_refs: Vec<String>,
    pub(crate) feature_group_refs: Vec<String>,
    pub(crate) feature_refs: Vec<String>,
    pub(crate) merge_refs: Vec<String>,
    pub(crate) skip_webview_install: bool,
    pub(crate) license: Option<PathBuf>,
    pub(crate) enable_elevated_update_task: bool,
    pub(crate) banner_path: Option<PathBuf>,
    pub(crate) dialog_image_path: Option<PathBuf>,
    pub(crate) fips_compliant: bool,
    /// MSI installer version in the format `major.minor.patch.build` (build is optional).
    ///
    /// Because a valid version is required for MSI installer, it will be derived from [`PackageSettings::version`] if this field is not set.
    ///
    /// The first field is the major version and has a maximum value of 255. The second field is the minor version and has a maximum value of 255.
    /// The third and fourth fields have a maximum value of 65,535.
    ///
    /// See <https://learn.microsoft.com/en-us/windows/win32/msi/productversion> for more info.
    pub version: Option<String>,
    /// A GUID upgrade code for MSI installer. This code **_must stay the same across all of your updates_**,
    /// otherwise, Windows will treat your update as a different app and your users will have duplicate versions of your app.
    ///
    /// By default, tauri generates this code by generating a Uuid v5 using the string `<productName>.exe.app.x64` in the DNS namespace.
    /// You can use Tauri's CLI to generate and print this code for you by running `tauri inspect wix-upgrade-code`.
    ///
    /// It is recommended that you set this value in your tauri config file to avoid accidental changes in your upgrade code
    /// whenever you want to change your product name.
    pub upgrade_code: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct MacOsSettings {
    pub(crate) frameworks: Option<Vec<String>>,
    pub(crate) minimum_system_version: Option<String>,
    pub(crate) license: Option<String>,
    pub(crate) exception_domain: Option<String>,
    pub(crate) signing_identity: Option<String>,
    pub(crate) provider_short_name: Option<String>,
    pub(crate) entitlements: Option<String>,
    pub(crate) info_plist_path: Option<PathBuf>,
    /// List of custom files to add to the application bundle.
    /// Maps the path in the Contents directory in the app to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Preserve the hardened runtime version flag, see <https://developer.apple.com/documentation/security/hardened_runtime>
    ///
    /// Settings this to `false` is useful when using an ad-hoc signature, making it less strict.
    pub hardened_runtime: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WindowsSettings {
    pub(crate) digest_algorithm: Option<String>,
    pub(crate) certificate_thumbprint: Option<String>,
    pub(crate) timestamp_url: Option<String>,
    pub(crate) tsp: bool,
    pub(crate) wix: Option<WixSettings>,
    pub(crate) icon_path: Option<PathBuf>,
    pub(crate) webview_install_mode: WebviewInstallMode,
    pub(crate) webview_fixed_runtime_path: Option<PathBuf>,
    pub(crate) allow_downgrades: bool,
    pub(crate) nsis: Option<NsisSettings>,
    /// Specify a custom command to sign the binaries.
    /// This command needs to have a `%1` in it which is just a placeholder for the binary path,
    /// which we will detect and replace before calling the command.
    ///
    /// Example:
    /// ```text
    /// sign-cli --arg1 --arg2 %1
    /// ```
    ///
    /// By Default we use `signtool.exe` which can be found only on Windows so
    /// if you are on another platform and want to cross-compile and sign you will
    /// need to use another tool like `osslsigncode`.
    pub sign_command: Option<CustomSignCommandSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NsisSettings {
    pub(crate) template: Option<PathBuf>,
    pub(crate) license: Option<PathBuf>,
    pub(crate) header_image: Option<PathBuf>,
    pub(crate) sidebar_image: Option<PathBuf>,
    pub(crate) installer_icon: Option<PathBuf>,
    pub(crate) install_mode: NSISInstallerMode,
    pub(crate) languages: Option<Vec<String>>,
    pub(crate) custom_language_files: Option<HashMap<String, PathBuf>>,
    pub(crate) display_language_selector: bool,
    pub(crate) start_menu_folder: Option<String>,
    pub(crate) installer_hooks: Option<PathBuf>,
    /// Try to ensure that the WebView2 version is equal to or newer than this version,
    /// if the user's WebView2 is older than this version,
    /// the installer will try to trigger a WebView2 update.
    pub minimum_webview2_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum NSISInstallerMode {
    CurrentUser,
    PerMachine,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum WebviewInstallMode {
    Skip,
    DownloadBootstrapper { silent: bool },
    EmbedBootstrapper { silent: bool },
    OfflineInstaller { silent: bool },
    FixedRuntime { path: PathBuf },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSignCommandSettings {
    /// The command to run to sign the binary.
    pub cmd: String,
    /// The arguments to pass to the command.
    ///
    /// "%1" will be replaced with the path to the binary to be signed.
    pub args: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PackageType {
    /// "macos"
    MacOsBundle,
    /// "ios"
    IosBundle,
    /// "msi"
    WindowsMsi,
    /// "nsis"
    Nsis,
    /// "deb"
    Deb,
    /// "rpm"
    Rpm,
    /// "appimage"
    AppImage,
    /// "dmg"
    Dmg,
    /// "updater"
    Updater,
}
````

<SYSTEM>This is the developer documentation for Dioxus from /learn/0.7/guides/deploy/.</SYSTEM>
# Publishing

After you have built your application, you will need to publish it somewhere. This reference will outline different methods of publishing your desktop or web application.

## Web: Publishing with GitHub Pages

Edit your `Dioxus.toml` to point your `out_dir` to the `docs` folder and the `base_path` to the name of your repo:

````toml
[application]
# ...
[web.app]
base_path = "your_repo"
````

Then build your app and publish it to Github:

* Make sure GitHub Pages is set up for your repo to publish any static files in the docs directory
* Build your app into the `docs` directory with:

````sh
dx bundle --out-dir docs
````

* Move the static content from `docs/public` to `docs`

````sh
mv docs/public/* docs
````

* Make a copy of your `docs/index.html` file and rename the copy to `docs/404.html` so that your app will work with client-side routing:

````sh
cp docs/index.html docs/404.html
````

* Add and commit with git
* Push to GitHub

## Desktop: Creating an installer

Dioxus desktop app uses your operating system's WebView library, so it's portable to be distributed for other platforms.

In this section, we'll cover how to bundle your app for macOS, Windows, and Linux.

## Preparing your application for bundling

Depending on your platform, you may need to add some additional code to your `main.rs` file to make sure your app is ready for bundling. On Windows, you'll need to add the `#![windows_subsystem = "windows"]` attribute to your `main.rs` file to hide the terminal window that pops up when you run your app. **If you're developing on Windows, only use this when bundling.** It will disable the terminal, so you will not get logs of any kind. You can gate it behind a feature, like so:

````toml
# Cargo.toml
[features]
bundle = []
````

And then your `main.rs`:

````rust
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
````

## Adding assets to your application

If you want to bundle assets with your application, you can either use them with the `manganis` crate (covered more in the [assets](../../essentials/ui/assets.md) page), or you can include them in your `Dioxus.toml` file:

````toml
[bundle]
# The list of files to include in the bundle. These can contain globs.
resources = ["main.css", "header.svg", "**/*.png"]
````

## Install `dioxus CLI`

The first thing we'll do is install the [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli). This extension to cargo will make it very easy to package our app for the various platforms.

To install, simply run

`cargo install dioxus-cli`

## Building

To bundle your application you can simply run `dx bundle --release` (also add `--features bundle` if you're using that, see the [this](#preparing-your-application-for-bundling) for more) to produce a final app with all the optimizations and assets builtin.

Once you've ran the command, your app should be accessible in `dist/bundle/`.

For example, a macOS app would look like this:

![Published App](/assets/static/publish.png)

Nice! And it's only 4.8 Mb – extremely lean!! Because Dioxus leverages your platform's native WebView, Dioxus apps are extremely memory efficient and won't waste your battery.

>
> Note: not all CSS works the same on all platforms. Make sure to view your app's CSS on each platform – or web browser (Firefox, Chrome, Safari) before publishing.
# Publishing

After you have built your application, you will need to publish it somewhere. This reference will outline different methods of publishing your desktop or web application.

## Web: Publishing with GitHub Pages

Edit your `Dioxus.toml` to point your `out_dir` to the `docs` folder and the `base_path` to the name of your repo:

````toml
[application]
# ...
[web.app]
base_path = "your_repo"
````

Then build your app and publish it to Github:

* Make sure GitHub Pages is set up for your repo to publish any static files in the docs directory
* Build your app into the `docs` directory with:

````sh
dx bundle --out-dir docs
````

* Move the static content from `docs/public` to `docs`

````sh
mv docs/public/* docs
````

* Make a copy of your `docs/index.html` file and rename the copy to `docs/404.html` so that your app will work with client-side routing:

````sh
cp docs/index.html docs/404.html
````

* Add and commit with git
* Push to GitHub

## Desktop: Creating an installer

Dioxus desktop app uses your operating system's WebView library, so it's portable to be distributed for other platforms.

In this section, we'll cover how to bundle your app for macOS, Windows, and Linux.

## Preparing your application for bundling

Depending on your platform, you may need to add some additional code to your `main.rs` file to make sure your app is ready for bundling. On Windows, you'll need to add the `#![windows_subsystem = "windows"]` attribute to your `main.rs` file to hide the terminal window that pops up when you run your app. **If you're developing on Windows, only use this when bundling.** It will disable the terminal, so you will not get logs of any kind. You can gate it behind a feature, like so:

````toml
# Cargo.toml
[features]
bundle = []
````

And then your `main.rs`:

````rust
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
````

## Adding assets to your application

If you want to bundle assets with your application, you can either use them with the `manganis` crate (covered more in the [assets](../../essentials/ui/assets.md) page), or you can include them in your `Dioxus.toml` file:

````toml
[bundle]
# The list of files to include in the bundle. These can contain globs.
resources = ["main.css", "header.svg", "**/*.png"]
````

## Install `dioxus CLI`

The first thing we'll do is install the [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/main/packages/cli). This extension to cargo will make it very easy to package our app for the various platforms.

To install, simply run

`cargo install dioxus-cli`

## Building

To bundle your application you can simply run `dx bundle --release` (also add `--features bundle` if you're using that, see the [this](#preparing-your-application-for-bundling) for more) to produce a final app with all the optimizations and assets builtin.

Once you've ran the command, your app should be accessible in `dist/bundle/`.

For example, a macOS app would look like this:

![Published App](/assets/static/publish.png)

Nice! And it's only 4.8 Mb – extremely lean!! Because Dioxus leverages your platform's native WebView, Dioxus apps are extremely memory efficient and won't waste your battery.

>
> Note: not all CSS works the same on all platforms. Make sure to view your app's CSS on each platform – or web browser (Firefox, Chrome, Safari) before publishing.


<SYSTEM>This is the developer documentation for Dioxus from /learn/0.7/tutorial/databases.</SYSTEM>
# Working with Databases

Our HotDog app is coming together nicely! We implemented a very simple backend that saves the user's favorite dog images to a local "dogs.txt" file.

In practice, you will likely want to store data in a proper database. Modern databases are *much* more powerful than a text file!

If you already have a good understanding of databases, jump ahead to the [section where we integrate Sqlite with HotDog](#adding-database-operations-to-hotdog).

## Choosing a Database

In today's age of app development, there are *many* databases to choose from, each with their own strengths, weaknesses, and tradeoffs to consider. For apps with just a few users, it is fine to select a "simpler" database that's easier to manage. For apps with many users, you might want to consider more advanced databases with additional tooling to meet stricter requirements.

Here is a (incomplete!) list of databases and a short summary of each:

* [PostgreSQL](https://www.postgresql.org): Advanced database known for its powerful plugin system.
* [MySQL](https://www.mysql.com): World's most popular open source database good for all apps.
* [SQLite](https://www.sqlite.org): Simple file-based engine known for its reliability and embeddability.
* [Oracle](https://www.oracle.com/database/): Advanced commercial database known for its enterprise features.
* [Redis](http://redis.io): Simple key-value database known for its great performance.
* [MongoDB](https://www.mongodb.com/): A database ideal for data that doesn't fit into rows and columns.
* [SurrealDB](https://surrealdb.com/): A new "all-in-one" database that combines many models.
* [CockroachDB](https://www.cockroachlabs.com): Distributed SQL database designed for high-availability.
* [and many more](https://dev.to/shreyvijayvargiya/list-of-45-databases-in-the-world-57e8)!

There are many different types of databases, each good at different tasks. These might include:

* **Relational**: traditional row/column/table approach.
* **Document**: storing unstructured or loosely structured blobs of data.
* **Timeseries**: storing and querying lots of data that changes over time.
* **Graph**: querying data based on its connections to other data.
* **Key-value**: storing just key-value pairs - a fast concurrent HashMap.
* **In-memory**: designed for low-latency operations usually used as a cache.
* **Embedded**: a database that is shipped *inside* your app.

For most apps - unless you have specific requirements - we recommend a mainstream relational database like PostgreSQL or MySQL.

 > 
 > 📣 PostgreSQL is currently a very interesting option: it can be extended to support time-series, vector, graph, search and geo-spatial data with plugins.

In some cases, you might want a database that's specific to *just one app instance* or the *user's machine*. In these cases, you'll want to use an embedded database like [SQLite](https://www.sqlite.org) or [RocksDB](https://rocksdb.org).

## Adding Database operations to HotDog

For *HotDog*, we're going to use Sqlite. *HotDog* is a very simple app and will only ever have one user: you!

To add sqlite functionality to *HotDog*, we'll pull in the `rusqlite` crate. Note that `rusqlite` is only meant to be compiled on the server, so we'll feature gate it behind the `"server"` feature in our Cargo.toml.

````toml
[dependencies]
# ....
rusqlite = { version = "0.32.1", optional = true } # <--- add rusqlite

[features]
# ....
server = ["dioxus/server", "dep:rusqlite"] # <---- add dep:rusqlite
````

To connect to our database, we're going to use the `rusqlite::Connection`. Rusqlite connections are not thread-safe and must exist once-per-thread, so we'll need to wrap it in a thread_local.

When the connection is initialized, we'll run a SQL action to create the "dogs" table with our data.

````rs@guide_databases.rs
// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}
````

Now, in our `save_dog` server function, we can use SQL to insert the value into the database:

````rs@guide_databases.rs
#[server]
async fn save_dog(image: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}
````

Once the app is launched, you should see a "hotdog.db" file in your crate's directory. Let's save a few dog photos and then open the database in a database viewer. If all goes well, you should see the saved dog photos!

![Databaseview](/assets/06_docs/hotdog-db-view.mp4)

## Notes on Databases and Rust

While there are many database providers, Rust support can be limited. Rust is still a new choice for web development. In this section we'll provide our own (biased!) opinions on what libraries we recommend for interacting with databases.

It's also important to note that several libraries exist at a higher level abstraction than raw SQL. These are called an *Object Relationship Mapper (ORM)*. Rust ORM libraries map the SQL language into ordinary Rust functions. We generally recommend just sticking with SQL, but ORMs can make working writing some queries easier.

* [Sqlx](https://github.com/launchbadge/sqlx): A straightforward yet large interface to Postgres, MySql, and Sqlite.
* [SeaORM](https://github.com/SeaQL/sea-orm): An ORM built on top of Sqlx for deriving databases.
* [rusqlite](https://github.com/rusqlite/rusqlite): An intuitive sqlite interface with no special ORM magic.
* [rust-postgres](https://github.com/sfackler/rust-postgres): An interface to Postgres with an API similar to rusqlite.
* [Turbosql](https://github.com/trevyn/turbosql): A *very* terse interface to Sqlite with automatic derives.

We aren't including libraries like [Diesel](http://diesel.rs) in this list since it seems that the Rust ecosystem has evolved towards newer projects with 1st-class async support.

There are many libraries we haven't tested yet, but might be worth checking out:

* [firebase-rs](https://github.com/emreyalvac/firebase-rs): Firebase client crate
* [postgrest-rs](https://github.com/supabase-community/postgrest-rs): Supabase client crate
* [mongo-rust-driver](https://github.com/mongodb/mongo-rust-driver): Official MongoDB client crate

## Choosing a Database Provider

While there are just a handful of databases you might consider for your app, there are many *database providers*, each with their own strengths and weaknesses. We are not sponsored by any of these providers - this is just a list of providers we have seen in use by Rust apps.

You *do not* need to use a database provider. Databases providers provide paid database hosting. It will cost you money to use these providers! Many have a free tier and some support "scale-to-zero" to help you save money on small apps. At any time, you are free to host and manage your own database.

For popular relational databases:

* [GCP](https://cloud.google.com/products/databases): Provides AlloyDB (enterprise postgres), CloudSQL (MySql, Postgres), and more.
* [AWS](https://aws.amazon.com/products/databases/): Provides RDS, Aurora, DynamoDB, and more.
* [PlanetScale](https://planetscale.com): Reliable MySQL-compatible database with sharding designed for scale.
* [Firebase](https://firebase.google.com): Google's comprehensive real-time database designed for rapid app development.
* [Supabase](https://supabase.com): Hosted Postgres known for its great dashboard and tooling.
* [Neon](https://neon.tech): Hosted Postgres that separates compute and storage for scale-to-zero apps.

For Sqlite:

* [LiteFS](https://fly.io/docs/litefs/): A distributed Sqlite sync engine designed to be used with Fly.io
* [Turso](https://turso.tech): A "multi-tenant" sqlite provider that maintains one isolated database per user

The "scale-to-zero" relational solutions:

* [AWS Aurora](https://aws.amazon.com/rds/aurora/)
* [LiteFS](https://fly.io/docs/litefs/)

We don't suggest any particular database provider.

* If you have lots of free cloud credits, consider AWS/GCP/Azure.
* If you want Postgres with a good dashboard, consider Supabase or Neon.
* If you want a simple experience, consider Turso or LiteFS.
# Working with Databases

Our HotDog app is coming together nicely! We implemented a very simple backend that saves the user's favorite dog images to a local "dogs.txt" file.

In practice, you will likely want to store data in a proper database. Modern databases are *much* more powerful than a text file!

If you already have a good understanding of databases, jump ahead to the [section where we integrate Sqlite with HotDog](#adding-database-operations-to-hotdog).

## Choosing a Database

In today's age of app development, there are *many* databases to choose from, each with their own strengths, weaknesses, and tradeoffs to consider. For apps with just a few users, it is fine to select a "simpler" database that's easier to manage. For apps with many users, you might want to consider more advanced databases with additional tooling to meet stricter requirements.

Here is a (incomplete!) list of databases and a short summary of each:

* [PostgreSQL](https://www.postgresql.org): Advanced database known for its powerful plugin system.
* [MySQL](https://www.mysql.com): World's most popular open source database good for all apps.
* [SQLite](https://www.sqlite.org): Simple file-based engine known for its reliability and embeddability.
* [Oracle](https://www.oracle.com/database/): Advanced commercial database known for its enterprise features.
* [Redis](http://redis.io): Simple key-value database known for its great performance.
* [MongoDB](https://www.mongodb.com/): A database ideal for data that doesn't fit into rows and columns.
* [SurrealDB](https://surrealdb.com/): A new "all-in-one" database that combines many models.
* [CockroachDB](https://www.cockroachlabs.com): Distributed SQL database designed for high-availability.
* [and many more](https://dev.to/shreyvijayvargiya/list-of-45-databases-in-the-world-57e8)!

There are many different types of databases, each good at different tasks. These might include:

* **Relational**: traditional row/column/table approach.
* **Document**: storing unstructured or loosely structured blobs of data.
* **Timeseries**: storing and querying lots of data that changes over time.
* **Graph**: querying data based on its connections to other data.
* **Key-value**: storing just key-value pairs - a fast concurrent HashMap.
* **In-memory**: designed for low-latency operations usually used as a cache.
* **Embedded**: a database that is shipped *inside* your app.

For most apps - unless you have specific requirements - we recommend a mainstream relational database like PostgreSQL or MySQL.

 > 
 > 📣 PostgreSQL is currently a very interesting option: it can be extended to support time-series, vector, graph, search and geo-spatial data with plugins.

In some cases, you might want a database that's specific to *just one app instance* or the *user's machine*. In these cases, you'll want to use an embedded database like [SQLite](https://www.sqlite.org) or [RocksDB](https://rocksdb.org).

## Adding Database operations to HotDog

For *HotDog*, we're going to use Sqlite. *HotDog* is a very simple app and will only ever have one user: you!

To add sqlite functionality to *HotDog*, we'll pull in the `rusqlite` crate. Note that `rusqlite` is only meant to be compiled on the server, so we'll feature gate it behind the `"server"` feature in our Cargo.toml.

````toml
[dependencies]
# ....
rusqlite = { version = "0.32.1", optional = true } # <--- add rusqlite

[features]
# ....
server = ["dioxus/server", "dep:rusqlite"] # <---- add dep:rusqlite
````

To connect to our database, we're going to use the `rusqlite::Connection`. Rusqlite connections are not thread-safe and must exist once-per-thread, so we'll need to wrap it in a thread_local.

When the connection is initialized, we'll run a SQL action to create the "dogs" table with our data.

````rs@guide_databases.rs
// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}
````

Now, in our `save_dog` server function, we can use SQL to insert the value into the database:

````rs@guide_databases.rs
#[server]
async fn save_dog(image: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}
````

Once the app is launched, you should see a "hotdog.db" file in your crate's directory. Let's save a few dog photos and then open the database in a database viewer. If all goes well, you should see the saved dog photos!

![Databaseview](/assets/06_docs/hotdog-db-view.mp4)

## Notes on Databases and Rust

While there are many database providers, Rust support can be limited. Rust is still a new choice for web development. In this section we'll provide our own (biased!) opinions on what libraries we recommend for interacting with databases.

It's also important to note that several libraries exist at a higher level abstraction than raw SQL. These are called an *Object Relationship Mapper (ORM)*. Rust ORM libraries map the SQL language into ordinary Rust functions. We generally recommend just sticking with SQL, but ORMs can make working writing some queries easier.

* [Sqlx](https://github.com/launchbadge/sqlx): A straightforward yet large interface to Postgres, MySql, and Sqlite.
* [SeaORM](https://github.com/SeaQL/sea-orm): An ORM built on top of Sqlx for deriving databases.
* [rusqlite](https://github.com/rusqlite/rusqlite): An intuitive sqlite interface with no special ORM magic.
* [rust-postgres](https://github.com/sfackler/rust-postgres): An interface to Postgres with an API similar to rusqlite.
* [Turbosql](https://github.com/trevyn/turbosql): A *very* terse interface to Sqlite with automatic derives.

We aren't including libraries like [Diesel](http://diesel.rs) in this list since it seems that the Rust ecosystem has evolved towards newer projects with 1st-class async support.

There are many libraries we haven't tested yet, but might be worth checking out:

* [firebase-rs](https://github.com/emreyalvac/firebase-rs): Firebase client crate
* [postgrest-rs](https://github.com/supabase-community/postgrest-rs): Supabase client crate
* [mongo-rust-driver](https://github.com/mongodb/mongo-rust-driver): Official MongoDB client crate

## Choosing a Database Provider

While there are just a handful of databases you might consider for your app, there are many *database providers*, each with their own strengths and weaknesses. We are not sponsored by any of these providers - this is just a list of providers we have seen in use by Rust apps.

You *do not* need to use a database provider. Databases providers provide paid database hosting. It will cost you money to use these providers! Many have a free tier and some support "scale-to-zero" to help you save money on small apps. At any time, you are free to host and manage your own database.

For popular relational databases:

* [GCP](https://cloud.google.com/products/databases): Provides AlloyDB (enterprise postgres), CloudSQL (MySql, Postgres), and more.
* [AWS](https://aws.amazon.com/products/databases/): Provides RDS, Aurora, DynamoDB, and more.
* [PlanetScale](https://planetscale.com): Reliable MySQL-compatible database with sharding designed for scale.
* [Firebase](https://firebase.google.com): Google's comprehensive real-time database designed for rapid app development.
* [Supabase](https://supabase.com): Hosted Postgres known for its great dashboard and tooling.
* [Neon](https://neon.tech): Hosted Postgres that separates compute and storage for scale-to-zero apps.

For Sqlite:

* [LiteFS](https://fly.io/docs/litefs/): A distributed Sqlite sync engine designed to be used with Fly.io
* [Turso](https://turso.tech): A "multi-tenant" sqlite provider that maintains one isolated database per user

The "scale-to-zero" relational solutions:

* [AWS Aurora](https://aws.amazon.com/rds/aurora/)
* [LiteFS](https://fly.io/docs/litefs/)

We don't suggest any particular database provider.

* If you have lots of free cloud credits, consider AWS/GCP/Azure.
* If you want Postgres with a good dashboard, consider Supabase or Neon.
* If you want a simple experience, consider Turso or LiteFS.

<SYSTEM>This is the developer documentation for Dioxus from /learn/0.7/guides/deploy/config.</SYSTEM>
### Bundling config

The `[bundle]` section of our Dioxus.toml can take a variety of options.

Here are the options, in the form of Rust structs.

````rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct BundleConfig {
    /// eg. com.dioxuslabs
    pub(crate) identifier: Option<String>,
    /// eg. DioxusLabs
    pub(crate) publisher: Option<String>,
    /// eg. assets/icon.png
    pub(crate) icon: Option<Vec<String>>,
    /// eg. Extra assets like "img.png"
    pub(crate) resources: Option<Vec<String>>,
    /// eg. DioxusLabs
    pub(crate) copyright: Option<String>,
    /// eg. "Social Media"
    pub(crate) category: Option<String>,
    /// eg. "A great social media app"
    pub(crate) short_description: Option<String>,
    /// eg. "A social media app that makes people love app development"
    pub(crate) long_description: Option<String>,
    /// eg. extra binaries (like tools) to include in the final app
    pub(crate) external_bin: Option<Vec<String>>,
    /// Additional debian-only settings (see below)
    pub(crate) deb: Option<DebianSettings>,
    /// Additional macos settings (see below)
    pub(crate) macos: Option<MacOsSettings>,
    /// Additional windows settings (see below)
    pub(crate) windows: Option<WindowsSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DebianSettings {
    // OS-specific settings:
    /// the list of debian dependencies.
    pub depends: Option<Vec<String>>,
    /// the list of dependencies the package provides.
    pub provides: Option<Vec<String>>,
    /// the list of package conflicts.
    pub conflicts: Option<Vec<String>>,
    /// the list of package replaces.
    pub replaces: Option<Vec<String>>,
    /// List of custom files to add to the deb package.
    /// Maps the path on the debian package to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Path to a custom desktop file Handlebars template.
    ///
    /// Available variables: `categories`, `comment` (optional), `exec`, `icon` and `name`.
    pub desktop_template: Option<PathBuf>,
    /// Define the section in Debian Control file. See : <https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections>
    pub section: Option<String>,
    /// Change the priority of the Debian Package. By default, it is set to `optional`.
    /// Recognized Priorities as of now are :  `required`, `important`, `standard`, `optional`, `extra`
    pub priority: Option<String>,
    /// Path of the uncompressed Changelog file, to be stored at /usr/share/doc/package-name/changelog.gz. See
    /// <https://www.debian.org/doc/debian-policy/ch-docs.html#changelog-files-and-release-notes>
    pub changelog: Option<PathBuf>,
    /// Path to script that will be executed before the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_install_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_install_script: Option<PathBuf>,
    /// Path to script that will be executed before the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_remove_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_remove_script: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct WixSettings {
    pub(crate) language: Vec<(String, Option<PathBuf>)>,
    pub(crate) template: Option<PathBuf>,
    pub(crate) fragment_paths: Vec<PathBuf>,
    pub(crate) component_group_refs: Vec<String>,
    pub(crate) component_refs: Vec<String>,
    pub(crate) feature_group_refs: Vec<String>,
    pub(crate) feature_refs: Vec<String>,
    pub(crate) merge_refs: Vec<String>,
    pub(crate) skip_webview_install: bool,
    pub(crate) license: Option<PathBuf>,
    pub(crate) enable_elevated_update_task: bool,
    pub(crate) banner_path: Option<PathBuf>,
    pub(crate) dialog_image_path: Option<PathBuf>,
    pub(crate) fips_compliant: bool,
    /// MSI installer version in the format `major.minor.patch.build` (build is optional).
    ///
    /// Because a valid version is required for MSI installer, it will be derived from [`PackageSettings::version`] if this field is not set.
    ///
    /// The first field is the major version and has a maximum value of 255. The second field is the minor version and has a maximum value of 255.
    /// The third and fourth fields have a maximum value of 65,535.
    ///
    /// See <https://learn.microsoft.com/en-us/windows/win32/msi/productversion> for more info.
    pub version: Option<String>,
    /// A GUID upgrade code for MSI installer. This code **_must stay the same across all of your updates_**,
    /// otherwise, Windows will treat your update as a different app and your users will have duplicate versions of your app.
    ///
    /// By default, tauri generates this code by generating a Uuid v5 using the string `<productName>.exe.app.x64` in the DNS namespace.
    /// You can use Tauri's CLI to generate and print this code for you by running `tauri inspect wix-upgrade-code`.
    ///
    /// It is recommended that you set this value in your tauri config file to avoid accidental changes in your upgrade code
    /// whenever you want to change your product name.
    pub upgrade_code: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct MacOsSettings {
    pub(crate) frameworks: Option<Vec<String>>,
    pub(crate) minimum_system_version: Option<String>,
    pub(crate) license: Option<String>,
    pub(crate) exception_domain: Option<String>,
    pub(crate) signing_identity: Option<String>,
    pub(crate) provider_short_name: Option<String>,
    pub(crate) entitlements: Option<String>,
    pub(crate) info_plist_path: Option<PathBuf>,
    /// List of custom files to add to the application bundle.
    /// Maps the path in the Contents directory in the app to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Preserve the hardened runtime version flag, see <https://developer.apple.com/documentation/security/hardened_runtime>
    ///
    /// Settings this to `false` is useful when using an ad-hoc signature, making it less strict.
    pub hardened_runtime: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WindowsSettings {
    pub(crate) digest_algorithm: Option<String>,
    pub(crate) certificate_thumbprint: Option<String>,
    pub(crate) timestamp_url: Option<String>,
    pub(crate) tsp: bool,
    pub(crate) wix: Option<WixSettings>,
    pub(crate) icon_path: Option<PathBuf>,
    pub(crate) webview_install_mode: WebviewInstallMode,
    pub(crate) webview_fixed_runtime_path: Option<PathBuf>,
    pub(crate) allow_downgrades: bool,
    pub(crate) nsis: Option<NsisSettings>,
    /// Specify a custom command to sign the binaries.
    /// This command needs to have a `%1` in it which is just a placeholder for the binary path,
    /// which we will detect and replace before calling the command.
    ///
    /// Example:
    /// ```text
    /// sign-cli --arg1 --arg2 %1
    /// ```
    ///
    /// By Default we use `signtool.exe` which can be found only on Windows so
    /// if you are on another platform and want to cross-compile and sign you will
    /// need to use another tool like `osslsigncode`.
    pub sign_command: Option<CustomSignCommandSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NsisSettings {
    pub(crate) template: Option<PathBuf>,
    pub(crate) license: Option<PathBuf>,
    pub(crate) header_image: Option<PathBuf>,
    pub(crate) sidebar_image: Option<PathBuf>,
    pub(crate) installer_icon: Option<PathBuf>,
    pub(crate) install_mode: NSISInstallerMode,
    pub(crate) languages: Option<Vec<String>>,
    pub(crate) custom_language_files: Option<HashMap<String, PathBuf>>,
    pub(crate) display_language_selector: bool,
    pub(crate) start_menu_folder: Option<String>,
    pub(crate) installer_hooks: Option<PathBuf>,
    /// Try to ensure that the WebView2 version is equal to or newer than this version,
    /// if the user's WebView2 is older than this version,
    /// the installer will try to trigger a WebView2 update.
    pub minimum_webview2_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum NSISInstallerMode {
    CurrentUser,
    PerMachine,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum WebviewInstallMode {
    Skip,
    DownloadBootstrapper { silent: bool },
    EmbedBootstrapper { silent: bool },
    OfflineInstaller { silent: bool },
    FixedRuntime { path: PathBuf },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSignCommandSettings {
    /// The command to run to sign the binary.
    pub cmd: String,
    /// The arguments to pass to the command.
    ///
    /// "%1" will be replaced with the path to the binary to be signed.
    pub args: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PackageType {
    /// "macos"
    MacOsBundle,
    /// "ios"
    IosBundle,
    /// "msi"
    WindowsMsi,
    /// "nsis"
    Nsis,
    /// "deb"
    Deb,
    /// "rpm"
    Rpm,
    /// "appimage"
    AppImage,
    /// "dmg"
    Dmg,
    /// "updater"
    Updater,
}
````
### Bundling config

The `[bundle]` section of our Dioxus.toml can take a variety of options.

Here are the options, in the form of Rust structs.

````rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct BundleConfig {
    /// eg. com.dioxuslabs
    pub(crate) identifier: Option<String>,
    /// eg. DioxusLabs
    pub(crate) publisher: Option<String>,
    /// eg. assets/icon.png
    pub(crate) icon: Option<Vec<String>>,
    /// eg. Extra assets like "img.png"
    pub(crate) resources: Option<Vec<String>>,
    /// eg. DioxusLabs
    pub(crate) copyright: Option<String>,
    /// eg. "Social Media"
    pub(crate) category: Option<String>,
    /// eg. "A great social media app"
    pub(crate) short_description: Option<String>,
    /// eg. "A social media app that makes people love app development"
    pub(crate) long_description: Option<String>,
    /// eg. extra binaries (like tools) to include in the final app
    pub(crate) external_bin: Option<Vec<String>>,
    /// Additional debian-only settings (see below)
    pub(crate) deb: Option<DebianSettings>,
    /// Additional macos settings (see below)
    pub(crate) macos: Option<MacOsSettings>,
    /// Additional windows settings (see below)
    pub(crate) windows: Option<WindowsSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct DebianSettings {
    // OS-specific settings:
    /// the list of debian dependencies.
    pub depends: Option<Vec<String>>,
    /// the list of dependencies the package provides.
    pub provides: Option<Vec<String>>,
    /// the list of package conflicts.
    pub conflicts: Option<Vec<String>>,
    /// the list of package replaces.
    pub replaces: Option<Vec<String>>,
    /// List of custom files to add to the deb package.
    /// Maps the path on the debian package to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Path to a custom desktop file Handlebars template.
    ///
    /// Available variables: `categories`, `comment` (optional), `exec`, `icon` and `name`.
    pub desktop_template: Option<PathBuf>,
    /// Define the section in Debian Control file. See : <https://www.debian.org/doc/debian-policy/ch-archive.html#s-subsections>
    pub section: Option<String>,
    /// Change the priority of the Debian Package. By default, it is set to `optional`.
    /// Recognized Priorities as of now are :  `required`, `important`, `standard`, `optional`, `extra`
    pub priority: Option<String>,
    /// Path of the uncompressed Changelog file, to be stored at /usr/share/doc/package-name/changelog.gz. See
    /// <https://www.debian.org/doc/debian-policy/ch-docs.html#changelog-files-and-release-notes>
    pub changelog: Option<PathBuf>,
    /// Path to script that will be executed before the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_install_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is unpacked. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_install_script: Option<PathBuf>,
    /// Path to script that will be executed before the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub pre_remove_script: Option<PathBuf>,
    /// Path to script that will be executed after the package is removed. See
    /// <https://www.debian.org/doc/debian-policy/ch-maintainerscripts.html>
    pub post_remove_script: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct WixSettings {
    pub(crate) language: Vec<(String, Option<PathBuf>)>,
    pub(crate) template: Option<PathBuf>,
    pub(crate) fragment_paths: Vec<PathBuf>,
    pub(crate) component_group_refs: Vec<String>,
    pub(crate) component_refs: Vec<String>,
    pub(crate) feature_group_refs: Vec<String>,
    pub(crate) feature_refs: Vec<String>,
    pub(crate) merge_refs: Vec<String>,
    pub(crate) skip_webview_install: bool,
    pub(crate) license: Option<PathBuf>,
    pub(crate) enable_elevated_update_task: bool,
    pub(crate) banner_path: Option<PathBuf>,
    pub(crate) dialog_image_path: Option<PathBuf>,
    pub(crate) fips_compliant: bool,
    /// MSI installer version in the format `major.minor.patch.build` (build is optional).
    ///
    /// Because a valid version is required for MSI installer, it will be derived from [`PackageSettings::version`] if this field is not set.
    ///
    /// The first field is the major version and has a maximum value of 255. The second field is the minor version and has a maximum value of 255.
    /// The third and fourth fields have a maximum value of 65,535.
    ///
    /// See <https://learn.microsoft.com/en-us/windows/win32/msi/productversion> for more info.
    pub version: Option<String>,
    /// A GUID upgrade code for MSI installer. This code **_must stay the same across all of your updates_**,
    /// otherwise, Windows will treat your update as a different app and your users will have duplicate versions of your app.
    ///
    /// By default, tauri generates this code by generating a Uuid v5 using the string `<productName>.exe.app.x64` in the DNS namespace.
    /// You can use Tauri's CLI to generate and print this code for you by running `tauri inspect wix-upgrade-code`.
    ///
    /// It is recommended that you set this value in your tauri config file to avoid accidental changes in your upgrade code
    /// whenever you want to change your product name.
    pub upgrade_code: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct MacOsSettings {
    pub(crate) frameworks: Option<Vec<String>>,
    pub(crate) minimum_system_version: Option<String>,
    pub(crate) license: Option<String>,
    pub(crate) exception_domain: Option<String>,
    pub(crate) signing_identity: Option<String>,
    pub(crate) provider_short_name: Option<String>,
    pub(crate) entitlements: Option<String>,
    pub(crate) info_plist_path: Option<PathBuf>,
    /// List of custom files to add to the application bundle.
    /// Maps the path in the Contents directory in the app to the path of the file to include (relative to the current working directory).
    pub files: HashMap<PathBuf, PathBuf>,
    /// Preserve the hardened runtime version flag, see <https://developer.apple.com/documentation/security/hardened_runtime>
    ///
    /// Settings this to `false` is useful when using an ad-hoc signature, making it less strict.
    pub hardened_runtime: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WindowsSettings {
    pub(crate) digest_algorithm: Option<String>,
    pub(crate) certificate_thumbprint: Option<String>,
    pub(crate) timestamp_url: Option<String>,
    pub(crate) tsp: bool,
    pub(crate) wix: Option<WixSettings>,
    pub(crate) icon_path: Option<PathBuf>,
    pub(crate) webview_install_mode: WebviewInstallMode,
    pub(crate) webview_fixed_runtime_path: Option<PathBuf>,
    pub(crate) allow_downgrades: bool,
    pub(crate) nsis: Option<NsisSettings>,
    /// Specify a custom command to sign the binaries.
    /// This command needs to have a `%1` in it which is just a placeholder for the binary path,
    /// which we will detect and replace before calling the command.
    ///
    /// Example:
    /// ```text
    /// sign-cli --arg1 --arg2 %1
    /// ```
    ///
    /// By Default we use `signtool.exe` which can be found only on Windows so
    /// if you are on another platform and want to cross-compile and sign you will
    /// need to use another tool like `osslsigncode`.
    pub sign_command: Option<CustomSignCommandSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NsisSettings {
    pub(crate) template: Option<PathBuf>,
    pub(crate) license: Option<PathBuf>,
    pub(crate) header_image: Option<PathBuf>,
    pub(crate) sidebar_image: Option<PathBuf>,
    pub(crate) installer_icon: Option<PathBuf>,
    pub(crate) install_mode: NSISInstallerMode,
    pub(crate) languages: Option<Vec<String>>,
    pub(crate) custom_language_files: Option<HashMap<String, PathBuf>>,
    pub(crate) display_language_selector: bool,
    pub(crate) start_menu_folder: Option<String>,
    pub(crate) installer_hooks: Option<PathBuf>,
    /// Try to ensure that the WebView2 version is equal to or newer than this version,
    /// if the user's WebView2 is older than this version,
    /// the installer will try to trigger a WebView2 update.
    pub minimum_webview2_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum NSISInstallerMode {
    CurrentUser,
    PerMachine,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum WebviewInstallMode {
    Skip,
    DownloadBootstrapper { silent: bool },
    EmbedBootstrapper { silent: bool },
    OfflineInstaller { silent: bool },
    FixedRuntime { path: PathBuf },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSignCommandSettings {
    /// The command to run to sign the binary.
    pub cmd: String,
    /// The arguments to pass to the command.
    ///
    /// "%1" will be replaced with the path to the binary to be signed.
    pub args: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PackageType {
    /// "macos"
    MacOsBundle,
    /// "ios"
    IosBundle,
    /// "msi"
    WindowsMsi,
    /// "nsis"
    Nsis,
    /// "deb"
    Deb,
    /// "rpm"
    Rpm,
    /// "appimage"
    AppImage,
    /// "dmg"
    Dmg,
    /// "updater"
    Updater,
}
````

<SYSTEM>This is the developer documentation for Dioxus from /learn/0.7/tutorial/bundle.</SYSTEM>
# Bundling

Congratulations! You built your first fully-functional Dioxus app, completely loaded with Routing, asynchronous data-fetching, Server Functions, and a database! That's incredible for just a few minutes of work.

Let's get your app bundled for multiple platforms and then ready to deploy.

## Testing on iOS

To test iOS, your development environment needs to be setup to build iOS apps. This involves a few steps:

* Make sure you are developing on a device running macOS
* Install XCode
* [Download a recent iOS SDK and Emulator pack](https://developer.apple.com/ios/)
* Install the iOS Rust toolchains (`aarch64-apple-ios aarch64-apple-ios-sim`)

This is a multi-step process and requires creating an Apple Developer account. You shouldn't need to pay any fees until you want to sign your app. Signing your app is required for deploying to the Apple App Store and testing on your iOS device.

Simply run run `dx serve --ios` and your app should load in the iOS Simulator.

![DogApp](/assets/06_docs/dog-app-ios.mp4)

Fantastic - our app works seamlessly with no changes.

## Testing on Android

Setting up your environment for Android development takes time, so make sure to read the [mobile tooling guide](../guides/platforms/mobile.md).

* Install the Android NDK and SDK
* Set JAVA_HOME, ANDROID_HOME, NDK_HOME, and fix PATH issues to use the `emulator` tool
* Install and set up an Android emulator
* Install the Android rustup targets (`aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`)

Getting your Android install correct might be somewhat tricky, so try enabling "verbose" mode on dx to debug any issues.

If all goes well, we can simply serve and our app should show up in our Android simulator.

````
dx serve --android
````

![Android DogApp](/assets/06_docs/android-dogapp.mp4)

## Testing on Desktop

HotDog also works on macOS, Windows, and Linux! We can use `dx serve --desktop` to serve our app as a desktop app.

![HotDogDesktop](/assets/06_docs/hotdog-desktop.png)

## Bundling for the web

After we're done making changes to our server and client apps, we can build bundles that are ready to distribute.

We're going to follow the same pattern as `dx serve` but with `dx bundle`. To start, let's build the web version of our app.

````sh
dx bundle --web
````

We should receive a series of INFO traces from the CLI as it builds, and then finally a path to the `public` folder it generates. Let's `cd` into its public directory and then check out its parent directory (cd ..) (the "web" folder).

````sh
❯ tree -L 3 --gitignore
.
├── public
│   ├── assets
│   │   ├── favicon.ico
│   │   ├── header.svg
│   │   ├── main-14aa55e73f669f3e.css
│   │   ├── main.css
│   │   └── screenshot.png
│   ├── index.html
│   └── wasm
│       ├── hot_dog.js
│       ├── hot_dog.js.br
│       ├── hot_dog_bg.wasm
│       ├── hot_dog_bg.wasm.br
│       └── snippets
└── server
````

`dx` built a `public` folder containing our assets, index.html, and various JavaScript snippets. Alongside our public folder is a `server` binary. When we deploy our web assets, we'll also want to deploy the server since it provides our server functions.

We can manually run the server simply by executing it. If you're using a default `dioxus::launch` setup, then the server will read the `IP` and `PORT` environment variables to serve.

>
> 📣 If you intend to serve from within a container (e.g., Docker), then you need to override the default `127.0.0.1` address with `IP=0.0.0.0` to listen for external connections.

![Serving the server](/assets/06_docs/serving_server.png)

## Bundling for Desktop and Mobile

To bundle desktop and mobile apps for deployment, we'll again use `dx bundle`. As of today, `dx bundle` only builds desktop apps for the native platform and architecture. Unfortunately, you can't build macOS apps from Windows, Linux apps from Mac, etc. We recommend using a Continuous Integration Matrix (like [Github Actions](https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/running-variations-of-jobs-in-a-workflow)) to perform a "cross-build" of your app in multiple different containers.

When bundling installable apps, there are many distribution formats to choose from. We can specify these formats using the `--package-types` flag on `dx bundle`. Dioxus supports packaging a broad number of package types:

* macOS: `.app`, `.dmg`
* Linux: `.appimage`, `.rpm`, `.deb`
* Windows: `.msi`, `.exe`
* iOS: `.app`
* Android: `.apk`

You can specify package types like so:

````sh
dx bundle --desktop \
    --package-types "macos" \
    --package-types "dmg"
````

Note that not all package-types are compatible with each platform - eg. only `.exe` can be built when specifying `--desktop`.

We should see the outputs in our terminal:

````sh
18.252s  INFO Bundled app successfully!
18.252s  INFO App produced 2 outputs:
18.252s  INFO app - [./target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]
18.252s  INFO dmg - [./target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]
````

Generally, you can distribute desktop apps without needing an app store. However, some platforms like macOS might require you to sign and notarize your application to be considered "safe" for your users to open.

When distributing mobile apps, you *are required* to sign and notarize your apps. Currently, Dioxus doesn't provide built-in utilities for this, so you'll need to figure out signing by reading 3rd-party documentation.

Tauri provides documentation on the signing process:

* [macOS](https://tauri.app/distribute/sign/macos/)
* [iOS](https://tauri.app/distribute/sign/iOS/)
* [Android](https://tauri.app/distribute/sign/android/)
* [Windows](https://tauri.app/distribute/sign/Windows/)
* [Linux](https://tauri.app/distribute/sign/Linux/)

## Customizing your Bundle

Before you ship your app, you might want to configure how your app icon looks, what entitlements it has, and other details. Our `dx bundle` tool can help you configure your bundles in a variety of ways.

To configure our bundle, we'll use our `Dioxus.toml` and modify the bundle section.

````toml
[application]
name = "docsite"

[bundle]
identifier = "com.dioxuslabs"
publisher = "DioxusLabs"
icon = ["assets/icon.png"]
````

For a full list of options, see the [reference page on the `bundle` section](../guides/deploy/config.md).

## Automating dx bundle with JSON mode

Also added in Dioxus 0.6 is a JSON output mode for `dx`. This makes it possible to parse the output of the CLI using tools like [jq](https://jqlang.github.io/jq/) which provide stdin/stdout support for JSON parsing.

This mode is not particular friendly to humans, but does contain more information than the standard trace output.

````sh
{"timestamp":"   9.927s","level":"INFO","message":"Bundled app successfully!","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"App produced 2 outputs:","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"DEBUG","message":"Bundling produced bundles: [\n    Bundle {\n        package_type: MacOsBundle,\n        bundle_paths: [\n            \"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\",\n        ],\n    },\n    Bundle {\n        package_type: Dmg,\n        bundle_paths: [\n            \"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg\",\n        ],\n    },\n]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"app - [/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"dmg - [/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"DEBUG","json":"{\"BundleOutput\":{\"bundles\":[\"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\",\"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg\"]}}","target":"dx"}
````

JSON mode works with all `dx` commands. However, it is most useful with `dx build` and `dx bundle`. The CLI always guarantees that the last emitted line is the result of the command. To collect the list of bundles from the `dx bundle` command, we can use `tail -1` and simple jq.

````sh
dx bundle --desktop \
    --json-output \
    --verbose \
    | tail -1 \
    | jq -r '.json | fromjson | .BundleOutput.bundles []'
````

This returns the list of bundles:

````
./target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app
./target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg
````
# Bundling

Congratulations! You built your first fully-functional Dioxus app, completely loaded with Routing, asynchronous data-fetching, Server Functions, and a database! That's incredible for just a few minutes of work.

Let's get your app bundled for multiple platforms and then ready to deploy.

## Testing on iOS

To test iOS, your development environment needs to be setup to build iOS apps. This involves a few steps:

* Make sure you are developing on a device running macOS
* Install XCode
* [Download a recent iOS SDK and Emulator pack](https://developer.apple.com/ios/)
* Install the iOS Rust toolchains (`aarch64-apple-ios aarch64-apple-ios-sim`)

This is a multi-step process and requires creating an Apple Developer account. You shouldn't need to pay any fees until you want to sign your app. Signing your app is required for deploying to the Apple App Store and testing on your iOS device.

Simply run run `dx serve --ios` and your app should load in the iOS Simulator.

![DogApp](/assets/06_docs/dog-app-ios.mp4)

Fantastic - our app works seamlessly with no changes.

## Testing on Android

Setting up your environment for Android development takes time, so make sure to read the [mobile tooling guide](../guides/platforms/mobile.md).

* Install the Android NDK and SDK
* Set JAVA_HOME, ANDROID_HOME, NDK_HOME, and fix PATH issues to use the `emulator` tool
* Install and set up an Android emulator
* Install the Android rustup targets (`aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`)

Getting your Android install correct might be somewhat tricky, so try enabling "verbose" mode on dx to debug any issues.

If all goes well, we can simply serve and our app should show up in our Android simulator.

````
dx serve --android
````

![Android DogApp](/assets/06_docs/android-dogapp.mp4)

## Testing on Desktop

HotDog also works on macOS, Windows, and Linux! We can use `dx serve --desktop` to serve our app as a desktop app.

![HotDogDesktop](/assets/06_docs/hotdog-desktop.png)

## Bundling for the web

After we're done making changes to our server and client apps, we can build bundles that are ready to distribute.

We're going to follow the same pattern as `dx serve` but with `dx bundle`. To start, let's build the web version of our app.

````sh
dx bundle --web
````

We should receive a series of INFO traces from the CLI as it builds, and then finally a path to the `public` folder it generates. Let's `cd` into its public directory and then check out its parent directory (cd ..) (the "web" folder).

````sh
❯ tree -L 3 --gitignore
.
├── public
│   ├── assets
│   │   ├── favicon.ico
│   │   ├── header.svg
│   │   ├── main-14aa55e73f669f3e.css
│   │   ├── main.css
│   │   └── screenshot.png
│   ├── index.html
│   └── wasm
│       ├── hot_dog.js
│       ├── hot_dog.js.br
│       ├── hot_dog_bg.wasm
│       ├── hot_dog_bg.wasm.br
│       └── snippets
└── server
````

`dx` built a `public` folder containing our assets, index.html, and various JavaScript snippets. Alongside our public folder is a `server` binary. When we deploy our web assets, we'll also want to deploy the server since it provides our server functions.

We can manually run the server simply by executing it. If you're using a default `dioxus::launch` setup, then the server will read the `IP` and `PORT` environment variables to serve.

>
> 📣 If you intend to serve from within a container (e.g., Docker), then you need to override the default `127.0.0.1` address with `IP=0.0.0.0` to listen for external connections.

![Serving the server](/assets/06_docs/serving_server.png)

## Bundling for Desktop and Mobile

To bundle desktop and mobile apps for deployment, we'll again use `dx bundle`. As of today, `dx bundle` only builds desktop apps for the native platform and architecture. Unfortunately, you can't build macOS apps from Windows, Linux apps from Mac, etc. We recommend using a Continuous Integration Matrix (like [Github Actions](https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/running-variations-of-jobs-in-a-workflow)) to perform a "cross-build" of your app in multiple different containers.

When bundling installable apps, there are many distribution formats to choose from. We can specify these formats using the `--package-types` flag on `dx bundle`. Dioxus supports packaging a broad number of package types:

* macOS: `.app`, `.dmg`
* Linux: `.appimage`, `.rpm`, `.deb`
* Windows: `.msi`, `.exe`
* iOS: `.app`
* Android: `.apk`

You can specify package types like so:

````sh
dx bundle --desktop \
    --package-types "macos" \
    --package-types "dmg"
````

Note that not all package-types are compatible with each platform - eg. only `.exe` can be built when specifying `--desktop`.

We should see the outputs in our terminal:

````sh
18.252s  INFO Bundled app successfully!
18.252s  INFO App produced 2 outputs:
18.252s  INFO app - [./target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]
18.252s  INFO dmg - [./target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]
````

Generally, you can distribute desktop apps without needing an app store. However, some platforms like macOS might require you to sign and notarize your application to be considered "safe" for your users to open.

When distributing mobile apps, you *are required* to sign and notarize your apps. Currently, Dioxus doesn't provide built-in utilities for this, so you'll need to figure out signing by reading 3rd-party documentation.

Tauri provides documentation on the signing process:

* [macOS](https://tauri.app/distribute/sign/macos/)
* [iOS](https://tauri.app/distribute/sign/iOS/)
* [Android](https://tauri.app/distribute/sign/android/)
* [Windows](https://tauri.app/distribute/sign/Windows/)
* [Linux](https://tauri.app/distribute/sign/Linux/)

## Customizing your Bundle

Before you ship your app, you might want to configure how your app icon looks, what entitlements it has, and other details. Our `dx bundle` tool can help you configure your bundles in a variety of ways.

To configure our bundle, we'll use our `Dioxus.toml` and modify the bundle section.

````toml
[application]
name = "docsite"

[bundle]
identifier = "com.dioxuslabs"
publisher = "DioxusLabs"
icon = ["assets/icon.png"]
````

For a full list of options, see the [reference page on the `bundle` section](../guides/deploy/config.md).

## Automating dx bundle with JSON mode

Also added in Dioxus 0.6 is a JSON output mode for `dx`. This makes it possible to parse the output of the CLI using tools like [jq](https://jqlang.github.io/jq/) which provide stdin/stdout support for JSON parsing.

This mode is not particular friendly to humans, but does contain more information than the standard trace output.

````sh
{"timestamp":"   9.927s","level":"INFO","message":"Bundled app successfully!","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"App produced 2 outputs:","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"DEBUG","message":"Bundling produced bundles: [\n    Bundle {\n        package_type: MacOsBundle,\n        bundle_paths: [\n            \"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\",\n        ],\n    },\n    Bundle {\n        package_type: Dmg,\n        bundle_paths: [\n            \"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg\",\n        ],\n    },\n]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"app - [/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"dmg - [/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"DEBUG","json":"{\"BundleOutput\":{\"bundles\":[\"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\",\"/Users/jonkelley/Development/Tinkering/06-demos/hot_dog/target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg\"]}}","target":"dx"}
````

JSON mode works with all `dx` commands. However, it is most useful with `dx build` and `dx bundle`. The CLI always guarantees that the last emitted line is the result of the command. To collect the list of bundles from the `dx bundle` command, we can use `tail -1` and simple jq.

````sh
dx bundle --desktop \
    --json-output \
    --verbose \
    | tail -1 \
    | jq -r '.json | fromjson | .BundleOutput.bundles []'
````

This returns the list of bundles:

````
./target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app
./target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg
````