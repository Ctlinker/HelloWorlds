# Notes on CSS

## Variables

### Syntax

```css
#myId, .myClass {
    --some-var: 10px;
}
```

### Usage

```css
#myId, .myClass {
    padding: var(--some-var);
}
```

### Variable Injection / Override

In programming, the term **variable** exists for a reason — it implies that the
value can _change_, depending on context. CSS Custom Properties (commonly known
as variables) follow this exact philosophy.

They're not just placeholders or constants. Their power lies in how they respond
to **scoping** and **cascading**, which gives them dynamic behavior — just like
variables in traditional programming.

Take this example:

```css
:root {
    --pulse-scale: 3;
}

@keyframes pulse {
    0%, 40%, 100% {
        transform: scale(0);
    }

    40%, 70% {
        transform: scale(var(--pulse-scale));
    }
}
```

This animation can be reused across components, elements, or classes. However,
let’s say you want to slightly _adjust_ the pulse effect for a specific part of
your layout — like a section or container:

```css
.my-spec-container {
    --pulse-scale: 1.5;
}

.btn {
    animation: pulse 0.3s linear infinite both;
}
```

Now, any `.btn` inside `.my-spec-container` will pulse at a `scale(1.5)`, while
those outside it will continue using the default `scale(3)` from `:root`.

This behavior is possible because CSS variables are **scoped**: A variable
declared on a parent element will cascade down to its children, and can override
global values in that local context.

---

This is one of those subtle features that, once understood, completely reshapes
how you think about CSS.

Far from being simple "find-and-replace" tokens, CSS variables allow for:

- **Global defaults**, like values defined in `:root`
- **Contextual overrides**, like adjustments made in a specific container
- **Component-level control**, without duplicating rules or redefining
  animations

It’s a powerful approach — almost like CSS borrowing concepts from
object-oriented or structured programming — where the _active value_ of a
variable depends on the element’s **scope** or **context**.

That’s what makes them true _variables_, and not just constants with a prettier
syntax.
