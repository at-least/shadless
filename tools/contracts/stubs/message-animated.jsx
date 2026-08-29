// message-animated stub: the real component drives motion/react preset
// animations — runtime behavior outside static oracle extraction. Render
// the wrapped Bubble/Message tree as-is (children pass-through), keeping
// the demo's static structure.
import * as React from "react"

export function MessageAnimated({ children, ...props }) {
  return React.createElement(React.Fragment, null, children)
}
