// Generic Flex component

import React from "react";
import styles from "./AppFlex.module.scss";

// Props for the AppFlex component
export interface AppFlexProps {
  children: React.ReactNode;
  id?: string;
  className?: string;
  flow?: "inline" | "block";
  direction?: "row" | "col";
  alignH?: "left" | "center" | "right" | "stretch";
  alignV?: "top" | "center" | "bottom" | "stretch";
  gap?: "none" | "tiny" | "small" | "medium" | "large";
}

// map nice human align names to css flex align names
const alignMap = {
  left: "flex-start",
  top: "flex-start",
  center: "center",
  right: "flex-end",
  bottom: "flex-end",
  stretch: "stretch",
};

// AppFlex component
export default function AppFlex({
  children,
  id = "",
  className = "",
  flow = "block",
  direction = "row",
  alignH = "center",
  alignV = "center",
  gap = "medium",
}: AppFlexProps) {
  const justifyContent = direction === "row" ? alignMap[alignH] : alignMap[alignV];
  const alignItems = direction === "row" ? alignMap[alignV] : alignMap[alignH];
  return (
    <div
      id={id}
      className={`${styles.flex}
      ${styles[flow]}
      ${styles[direction]}
      ${styles[gap]}
      ${className}`}
      style={{ justifyContent, alignItems }}
    >
      {children}
    </div>
  );
}
