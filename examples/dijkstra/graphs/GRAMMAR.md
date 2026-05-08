# Graph File Format — Grammar Specification

The Dijkstra example graph files (`small.json`, `medium_*.json`, `large_*.json`) use a deterministic, constrained subset of RFC 8259 JSON. This document is the authoritative specification of that format.

Two layers are defined:

1. **Contract grammar (input)** — what conformant readers must accept. Whitespace-tolerant, matches a subset of valid JSON.
2. **Canonical output (writer)** — what the `graphgen` tool emits, byte-for-byte. Pinned indentation, key order, and trailing newline. The canonical output is one specific instance of the contract grammar.

Any RFC 8259-conformant JSON parser will also accept files in this format, so language-stdlib JSON libraries (TypeScript `JSON.parse`, Go `encoding/json`, Rust `serde_json`, Zig `std.json`) work out of the box. The C++ example parses the same files with a small hand-rolled parser, demonstrating that the constrained format does not require a JSON library.

## 1. Contract grammar (ISO/IEC 14977 EBNF)

```ebnf
(* graph-file.ebnf — Dijkstra example graph format *)
(* A constrained, deterministic subset of RFC 8259 JSON. *)

graph_file    = ws , "{" , ws ,
                  '"vertices"' , ws , ":" , ws , vertex_array , ws , "," , ws ,
                  '"edges"'    , ws , ":" , ws , edge_array   , ws ,
                "}" , ws ;

vertex_array  = "[" , ws , [ vertex_id , { ws , "," , ws , vertex_id } ] , ws , "]" ;

edge_array    = "[" , ws , [ edge      , { ws , "," , ws , edge      } ] , ws , "]" ;

edge          = "{" , ws ,
                  '"from"'   , ws , ":" , ws , vertex_id , ws , "," , ws ,
                  '"to"'     , ws , ":" , ws , vertex_id , ws , "," , ws ,
                  '"weight"' , ws , ":" , ws , integer   , ws ,
                "}" ;

vertex_id     = '"' , id_char , { id_char } , '"' ;

id_char       = letter | digit | "_" ;

integer       = [ "-" ] , digit , { digit } ;

letter        = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J"
              | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T"
              | "U" | "V" | "W" | "X" | "Y" | "Z"
              | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j"
              | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t"
              | "u" | "v" | "w" | "x" | "y" | "z" ;

digit         = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;

ws            = { " " | "\t" | "\n" | "\r" } ;
```

### Properties guaranteed by this grammar

| Property | Rationale |
|---|---|
| Top-level object has exactly two keys: `vertices`, then `edges` | Hand-rolled parsers can match keys positionally |
| Each `edge` object has exactly three keys: `from`, `to`, `weight` (in that order) | No key dispatch logic needed |
| Vertex IDs are non-empty `[A-Za-z0-9_]+` strings | No JSON string escapes (`\n`, `\"`, `\u…`) ever |
| Weights are decimal integers, optionally negative | No float parsing, no scientific notation, no `NaN`/`Infinity` |
| ASCII only | No UTF-8 multi-byte handling |
| Empty arrays are valid | `vertices: []` and `edges: []` parse (degenerate but legal) |
| Whitespace permissive between tokens | Readers tolerate any RFC 8259 whitespace |

## 2. Canonical output (writer specification)

The `graphgen` tool emits canonical output: one specific layout of the contract grammar, pinned byte-for-byte. The `graphgen verify` subcommand re-emits each graph and `diff`s against the committed file; any drift is a regression.

### Byte-level rules

- **Encoding:** UTF-8, no BOM (file is ASCII, but the encoding is declared UTF-8 for tooling).
- **Line endings:** `\n` (LF). Not `\r\n`, regardless of platform.
- **Indentation:** 2 spaces. No tabs.
- **Trailing newline:** the file ends with exactly one `\n`.
- **Top-level layout:**
  ```
  {
    "vertices": [<vertex_id>, <vertex_id>, ...],
    "edges": [
      {"from": <vertex_id>, "to": <vertex_id>, "weight": <integer>},
      {"from": <vertex_id>, "to": <vertex_id>, "weight": <integer>}
    ]
  }
  ```
  - `"vertices"` array is on a single line — vertex IDs are short and there are at most a few thousand.
  - `"edges"` array is multiline — one edge per line, indented 4 spaces.
  - Spacing inside each edge object: `{"from": "X", "to": "Y", "weight": N}` with a single space after each `:` and `,`, no trailing space before `}`.
- **Key order (fixed):** top-level `vertices` then `edges`; each edge has `from`, `to`, `weight` in that order.
- **No trailing commas.** No comments. No duplicate keys.

### Vertex ID generation (writer convention)

`graphgen` emits vertex IDs as `v0`, `v1`, …, `vN-1` (zero-padded only when needed for sort stability — currently not). The hand-curated `small.json` uses single uppercase letters (`A`–`F`); both forms satisfy the grammar.

## 3. Reader and writer obligations

### Writers MUST

- Emit canonical output as specified in §2.
- Use only ASCII characters in vertex IDs.
- Emit integer weights only.
- Preserve key order.

### Writers MAY

- Use any vertex-ID naming scheme consistent with the `id_char` charset.

### Readers MUST

- Accept any input matching §1 (the contract grammar).
- Reject input that violates the grammar.

### Readers MAY

- Accept additional whitespace, key order variations, or other RFC 8259 features beyond the contract grammar (e.g., when delegating to a stdlib JSON library). This produces a stricter writer / more permissive reader pairing — the safe default.

## 4. Annotated example: `small.json`

```json
{
  "vertices": ["A", "B", "C", "D", "E", "F"],
  "edges": [
    {"from": "A", "to": "B", "weight": 6},
    {"from": "A", "to": "C", "weight": 4},
    {"from": "B", "to": "C", "weight": 2},
    {"from": "B", "to": "D", "weight": 2},
    {"from": "C", "to": "D", "weight": 1},
    {"from": "C", "to": "E", "weight": 2},
    {"from": "D", "to": "F", "weight": 7},
    {"from": "E", "to": "D", "weight": 1},
    {"from": "E", "to": "F", "weight": 3}
  ]
}
```

- Top-level object opens on line 1, closes on the last line.
- Vertices array is single-line (6 IDs).
- Edges array is multiline, one edge per line, 4-space indented.
- Vertex IDs are single uppercase letters — within the `id_char` charset.
- All weights are positive integers.
- File ends with a single `\n` after `}`.

This file is the canonical reference output of the format.

## 5. Schema evolution policy

The grammar is intentionally rigid to keep parsers simple. Future extensions (e.g., directed/undirected toggle, vertex properties, edge labels, floating-point weights) require:

1. **A version field.** Add `"version": <integer>` as the first key of the top-level object. Files without `version` are implicitly version 1 (the current grammar).
2. **A grammar update** (this document) describing the new version's contract.
3. **Reader migration.** Existing parsers must be updated to switch on `version` or to reject newer versions clearly.

The current grammar reserves `version` as a future key but does not require or accept it. Adding it is an explicit breaking change that must be coordinated across all five language readers and the C++ hand-rolled parser.

## References

- ISO/IEC 14977:1996 — *Information technology — Syntactic metalanguage — Extended BNF*
- RFC 8259 — *The JavaScript Object Notation (JSON) Data Interchange Format*
