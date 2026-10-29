((stem_macro
  (macro_name) @_latex
  (attr) @injection.content)
  (#eq? @_latex "latexmath")
  (#set! injection.language "latex"))

((stem_macro
  (macro_name) @_asciimath
  (attr) @injection.content)
  (#any-of? @_asciimath "stem" "asciimath")
  (#set! injection.language "asciimath"))

; `pass:[…]` reaches the output untouched, which for the HTML backend is HTML.
((macro_passthrough
  (attr) @injection.content)
  (#set! injection.language "HTML"))
