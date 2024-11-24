/// <reference types="vite-plugin-svgr/client" />

import classes from "./Header.module.css";
import Logo from "@/assets/gearclock.svg?react";

const Header = () => (
  <div className={classes.header}>
    <Logo className={classes.logo}></Logo>
    {/* <h1>WhenWorks</h1> */}
    <div className={classes.title}>WhenWorks</div>
    <div className={classes.logo}></div>
  </div>
);

export default Header;
