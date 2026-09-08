"use client";
import * as React from "react";
import { Direction } from "radix-ui";
function DirectionProvider({
  dir,
  direction,
  children
}) {
  return /* @__PURE__ */ React.createElement(Direction.DirectionProvider, { dir: direction ?? dir }, children);
}
const useDirection = Direction.useDirection;
export {
  DirectionProvider,
  useDirection
};
