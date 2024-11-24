/// <reference types="vite-plugin-svgr/client" />

import { InputHTMLAttributes } from "react";
import XIcon from "@/assets/x.svg?react";
import classes from "./Textbox.module.css";

type Props = {
  value: string;
  onChange: (value: string) => void;
} & Omit<InputHTMLAttributes<HTMLInputElement>, "onChange">;

const Textbox = ({ type, value, onChange, ...props }: Props) => (
  <div className={classes.wrapper}>
    <input
      type={type || "text"}
      className={classes.input}
      {...props}
      value={value}
      onChange={(event) => onChange(event.target.value)}
      autoComplete="off"
      autoCorrect="off"
      autoCapitalize="off"
      spellCheck="false"
      required={true}
    />
    <button className={classes.button} data-tooltip="Clear" onClick={() => onChange("")}>
      <XIcon />
    </button>
  </div>
);

export default Textbox;
