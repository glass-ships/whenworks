import {
  IconName,
  IconPrefix,
  //SizeProp,
  findIconDefinition,
} from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon as Icon } from "@fortawesome/react-fontawesome";
import styles from "./AppIcon.module.scss";
import "@/global/icons";

export type AppIconProps = {
  icon: string;
  // size?: SizeProp;
  size?: "tiny" | "small" | "medium" | "large";
  color?: string;
  className?: string;
  background?: boolean;
};

function getFontAwesomeIcon(iconName: string) {
  for (const prefix of ["fas", "far", "fab"]) {
    const match = findIconDefinition({
      prefix: prefix as IconPrefix,
      iconName: iconName as IconName,
    });
    if (match) return match;
  }
  return null;
}

export default function AppIcon({
  icon = "",
  // size = "1x",
  size = "small",
  color = "yellow",
  className = "",
  background = false,
}: AppIconProps) {
  const iconDef = getFontAwesomeIcon(icon);
  if (!iconDef) return null;
  return (
    <Icon
      icon={iconDef}
      // size={size as SizeProp}
      color={color}
      // fa-fw
      className={`
      ${styles.appicon} ${styles[size]}
      ${background ? styles.background : ""}
      ${className}
    `}
    />
  );
}
