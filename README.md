# Leptos Heroicons

[Heroicons](https://heroicons.com/) Components for [Leptos](https://leptos.dev/)

## Usage

For each icon you want to use, you'll need to add a feature flag for those components. Only the icons you specify will be compiled into your application.

```toml

leptos_heroicons = { git = "https://github.com/bbstilson/leptos_heroicons.git", features = [
    "24-solid-power",
    "24-outline-wrench",
    "20-solid-credit-card"
]}
```

You can also include all the icons of a given type by specifying the `size-type` feature:

```toml
leptos_heroicons = { git = "https://github.com/bbstilson/leptos_heroicons.git", features = [
    "24-solid",
    "24-outline",
    "20-solid",
    "16-solid"
]}
```

In your leptos app:

```rust
use leptos::prelude::*;
use leptos_heroicons::size_24::solid::Power;

#[component]
fn UnlimitedPower() -> impl IntoView {
    view! { <Power /> }
}

// For html attributes you can use the spread syntax
#[component]
fn HiddenPower() -> impl IntoView {
    view! { <Power class="hidden" {..} aria-hidden="true" /> }
}

```

### Why Git and not Cargo version?

See [this](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html) blog post from the Rust team. TL;DR: The max number of features any crate can publish is 300, and this library would need to publish around 1200. While the Rust team sorts out how best to support a large number of crate features, you will need to depend on the repo directly.

## Inspiration / Credit

- [Heroicons](https://heroicons.com/)
- [Relm4 Icons](https://github.com/Relm4/icons)
- [Yew Heroicons](https://git.sr.ht/~ankhers/yew_heroicons).
