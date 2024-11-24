import React, { useCallback, useEffect, useRef } from "react";
import AppButton from "./AppButton";
import styles from "./AppModal.module.scss";

export interface AppModalProps {
  isVisible?: boolean;
  onToggle?: (e?: any) => void;
  children?: React.ReactNode;
}

export default function AppModal({ isVisible, onToggle, children }: AppModalProps) {
  const modalRef = useRef<HTMLDivElement | null>(null);

  const toggleModal = useCallback(() => {
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    onToggle ? onToggle() : (isVisible = false);
  }, [onToggle]);

  const handleClickOutside = useCallback(
    (event: any) => {
      if (modalRef.current && !modalRef.current.contains(event.target)) {
        event.stopPropagation();
        toggleModal();
      }
    },
    [toggleModal]
  );

  const handleKeyDown = useCallback(
    (event: KeyboardEvent | React.KeyboardEvent<HTMLDivElement>) => {
      if (event.key === "Escape") {
        event.stopPropagation();
        toggleModal();
      }
    },
    [toggleModal]
  );

  useEffect(() => {
    if (isVisible) {
      document.addEventListener("mousedown", handleClickOutside);
      document.addEventListener("keydown", handleKeyDown);
    }
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isVisible, handleClickOutside, handleKeyDown]);

  /** Conditional Render */
  return isVisible ?
      <div ref={modalRef} className={styles.modalBackdrop} onClick={onToggle}>
        <div
          className={styles.modalContainer}
          onClick={(e) => {
            e.stopPropagation();
          }}
          onKeyDown={(e) => {
            e.stopPropagation();
          }}
        >
          {children}
          <AppButton onClick={onToggle} icon="xmark" design="circle" id="modal-close" />
        </div>
      </div>
    : null;

  /** Toggle CSS Display */
  return (
    <div
      ref={modalRef}
      className={styles.modalBackdrop}
      onClick={toggleModal}
      style={{ display: isVisible ? "flex" : "none" }}
      aria-modal="true"
      aria-hidden={!isVisible}
    >
      <div
        className={styles.modalContainer}
        onClick={(e) => {
          e.stopPropagation();
        }}
      >
        {children}
        <AppButton onClick={toggleModal} icon="xmark" design="circle" id="modal-close" />
      </div>
    </div>
  );
}
