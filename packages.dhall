{-
Welcome to your new Dhall package-set!

Below are instructions for how to edit this file for most use
cases, so that you don't need to know Dhall to use it.

## Use Cases

Most will want to do one or both of these options:
1. Override/Patch a package's dependency
2. Add a package not already in the default package set

This file will continue to work whether you use one or both options.
Instructions for each option are explained below.

### Overriding/Patching a package

Purpose:
- Change a package's dependency to a newer/older release than the
    default package set's release
- Use your own modified version of some dependency that may
    include new API, changed API, removed API by
    using your custom git repo of the library rather than
    the package set's repo

Syntax:
where `entityName` is one of the following:
- dependencies
- repo
- version
-------------------------------
let upstream = --
in  upstream
  with packageName.entityName = "new value"
-------------------------------

Example:
-------------------------------
let upstream = --
in  upstream
  with halogen.version = "master"
  with halogen.repo = "https://example.com/path/to/git/repo.git"

  with halogen-vdom.version = "v4.0.0"
  with halogen-vdom.dependencies = [ "extra-dependency" ] # halogen-vdom.dependencies
-------------------------------

### Additions

Purpose:
- Add packages that aren't already included in the default package set

Syntax:
where `<version>` is:
- a tag (i.e. "v4.0.0")
- a branch (i.e. "master")
- commit hash (i.e. "701f3e44aafb1a6459281714858fadf2c4c2a977")
-------------------------------
let upstream = --
in  upstream
  with new-package-name =
    { dependencies =
       [ "dependency1"
       , "dependency2"
       ]
    , repo =
       "https://example.com/path/to/git/repo.git"
    , version =
        "<version>"
    }
-------------------------------

Example:
-------------------------------
let upstream = --
in  upstream
  with benchotron =
      { dependencies =
          [ "arrays"
          , "exists"
          , "profunctor"
          , "strings"
          , "quickcheck"
          , "lcg"
          , "transformers"
          , "foldable-traversable"
          , "exceptions"
          , "node-fs"
          , "node-buffer"
          , "node-readline"
          , "datetime"
          , "now"
          ]
      , repo =
          "https://github.com/hdgarrood/purescript-benchotron.git"
      , version =
          "v7.0.0"
      }
-------------------------------
-}
let upstream =
      https://github.com/purescript/package-sets/releases/download/psc-0.15.10-20230919/packages.dhall
        sha256:d35b59ed7f44305db3f750a04ba3a1821900054f9b81a07985bf334995ac5d86

in  upstream
  with lumi-components =
    { dependencies =
        [ "aff"
        , "aff-coroutines"
        , "arrays"
        , "bifunctors"
        , "colors"
        , "console"
        , "control"
        , "coroutines"
        , "datetime"
        , "effect"
        , "either"
        , "enums"
        , "exceptions"
        , "fixed-precision"
        , "foldable-traversable"
        , "foreign"
        , "foreign-object"
        , "free"
        , "heterogeneous"
        , "integers"
        , "js-timers"
        , "js-uri"
        , "maybe"
        , "media-types"
        , "newtype"
        , "nonempty"
        , "nullable"
        , "numbers"
        , "ordered-collections"
        , "parallel"
        , "partial"
        , "prelude"
        , "profunctor-lenses"
        , "react-basic"
        , "react-basic-classic"
        , "react-basic-dnd"
        , "react-basic-dom"
        , "react-basic-emotion"
        , "react-basic-hooks"
        , "record"
        , "refs"
        , "simple-json"
        , "st"
        , "strings"
        , "tailrec"
        , "transformers"
        , "tuples"
        , "unsafe-coerce"
        , "unsafe-reference"
        , "web-dom"
        , "web-events"
        , "web-file"
        , "web-html"
        , "web-storage"
        , "web-uievents"
        ]
    , repo =
        "https://github.com/lumihq/purescript-lumi-components"
    , version =
        "v18.0.0"
    }