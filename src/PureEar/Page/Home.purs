module PureEar.Page.Home
  ( HomeProps
  , mkHome
  ) where

import Prelude

import Data.Map (size)
import Lumi.Components.Button (button, primary)
import Lumi.Components.Size (Size(..))
import React.Basic.DOM as DOM
import React.Basic.DOM.Events (capture_)
import React.Basic.Hooks (Component, component, useState, (/\))
import React.Basic.Hooks as React

type HomeProps
  = Unit

mkHome :: Component HomeProps
mkHome = do
  component "Home" \_props -> React.do
    counter /\ setCounter <- useState 0
    pure
      $ DOM.div
          { children:
              [ DOM.h1_ [ DOM.text "Functional Ear Trainer!" ]
              , DOM.h3_ [ DOM.text "Try clicking that button!" ]
              , DOM.h4_ [ DOM.text $ "You clicked the button " <> show counter <> " times" ]
              , button
                  primary
                    { title = "Clicks: " <> show counter
                    , size = ExtraLarge
                    , onPress =
                      capture_ do
                        setCounter (_ + 1)
                    }
              ]
          }
