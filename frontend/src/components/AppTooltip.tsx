/** Tooltip component to show content on hover */

import React from "react";
import styles from "./AppTooltip.module.scss";

// Props and Types for the AppTooltip component
export interface AppTooltipProps {
  children: React.ReactNode;
  content: React.ReactNode;
  position: "top" | "bottom" | "left" | "right";
}

// AppTooltip component
export default function AppTooltip({ children, content, position = "top" }: AppTooltipProps) {
  return (
    <div className={styles["tooltip-trigger"]}>
      {children}
      <div className={`${styles.tooltip} ${styles[`tooltip-${position}`]}`}>{content}</div>
    </div>
  );
}
