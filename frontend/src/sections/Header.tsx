import classes from "./Header.module.css";
import { ReactComponent as Logo } from "@/assets/gearclock.svg";

const Header = () => (
  <div className={classes.header}>
    <Logo className={classes.logo}></Logo>
    {/* <h1>WhenWorks</h1> */}
    <div className={classes.title}>WhenWorks</div>
    <div className={classes.logo}></div>
  </div>
);

export default Header;
