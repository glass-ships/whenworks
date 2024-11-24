import AppFlex from "./AppFlex";
import AppIcon from "./AppIcon";
import AppTooltip from "./AppTooltip";
import styles from "./Footer.module.scss";

export default function Footbar() {
  return (
    <footer className={styles.footer}>
      <AppFlex flow="inline" alignH="stretch" className={styles.social}>
        {[
          {
            icon: "github-alt",
            label: "GitHub",
            link: "https://github.com/glass-ships",
          },
          {
            icon: "linkedin",
            label: "LinkedIn",
            link: "https://www.linkedin.com/in/glass-ships",
          },
          {
            icon: "envelope",
            label: "Email",
            link: "mailto:contact@glass-ships.com",
          },
        ].map((item, index) => (
          <AppTooltip key={index} content={item.label} position="top">
            <a href={item.link} target="_blank">
              <AppIcon icon={item.icon} size="small" color="#facc15" />
            </a>
          </AppTooltip>
        ))}
      </AppFlex>

      <div>
        <p className={styles.license}>
          Created with love by{" "}
          <a href="https://github.com/glass-ships" target="_blank" rel="noopener noreferrer">
            Glass Ships &copy; {new Date().getFullYear()}
          </a>
        </p>
      </div>
    </footer>
  );
}
