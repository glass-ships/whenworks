// Generic grid/gallery component that displays a list of items in a grid

import React from "react";
import styles from "./AppGrid.module.scss";

// Props and Types for the AppGrid component
export interface AppGridProps {
  cols?: number;
  children?: React.ReactNode;
}

// AppGrid component
export default function AppGrid({ cols = 3, children }: AppGridProps) {
  const items = React.Children.toArray(children);
  return (
    <div
      className={`${styles.gallery} cols-${cols}`}
      style={
        {
          "--max-cols": cols.toString(),
          "--content-cols": Math.min(React.Children.count(children), cols).toString(),
        } as React.CSSProperties
      }
    >
      {items.map((item, index) => (
        <div key={index} className={styles.item}>
          {item}
        </div>
      ))}
    </div>
  );
}
