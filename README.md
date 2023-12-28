# Leptos Heroicons

[Heroicons](https://heroicons.com/) Components for [Leptos](https://leptos.dev/)

## Usage

For each icon you want to use, you'll need to add a feature flag for those components. Only the icons you specify will be compiled into your application.

```toml
leptos_heroicons = { version = "0.2.0", features = [
    "24-solid-power",
    "24-outline-wrench",
    "20-solid-credit-card"
]}
```

You can also include all the icons of a given type by specifying the `size-type` feature:

```toml
leptos_heroicons = { version = "0.2.0", features = [
    "24-solid",
    "24-outline",
    "20-solid",
    "16-solid"
]}
```

In your leptos app:

```rust
use leptos::*;
use leptos_heroicons::size_24::solid::Power;

#[component]
fn UnlimitedPower() -> impl IntoView {
    view! { <Power /> }
}
```

## Inspiration / Credit

- [Heroicons](https://heroicons.com/)
- [Relm4 Icons](https://github.com/Relm4/icons)
- [Yew Heroicons](https://git.sr.ht/~ankhers/yew_heroicons).
