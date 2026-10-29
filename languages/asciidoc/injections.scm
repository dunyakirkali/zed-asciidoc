((paragraph) @injection.content
  (#set! injection.language "AsciiDoc Inline"))

((line) @injection.content
  (#set! injection.language "AsciiDoc Inline"))

; Inject comment
((line_comment) @injection.content
  (#set! injection.language "comment"))

((block_comment) @injection.content
  (#set! injection.language "comment"))

; Source listing: `[source,ruby]` - the language is the second positional.
((section_block
  (element_attr
    (positional_attr
      (block_style) @_source)
    (positional_attr) @injection.language)
  (listing_block
    (listing_block_start_marker)
    (listing_block_body) @injection.content
    (listing_block_end_marker)))
  (#eq? @_source "source"))
