import { useState } from 'react';
import ReactModal from 'react-modal';
import { InputResolutionNames } from '../Resolution';
import styles from './PlaygroundRenderSettings.module.css';
import SettingsInputs from './SettingsInputs';

interface PlaygroundRenderSettingsProps {
  onSubmit: () => Promise<void>;
  onChange: (string, ResolutionName) => void;
  selectedInputResolutionNames: InputResolutionNames;
}

function PlaygroundRenderSettings({
  onSubmit,
  onChange,
  selectedInputResolutionNames,
}: PlaygroundRenderSettingsProps) {
  const [inputsSettingsModalOpen, setInputsSettingsModalOpen] = useState(false);

  return (
    <div style={{ margin: '10px' }}>
      <button
        className="button button--outline button--secondary"
        onClick={() => {
          setInputsSettingsModalOpen(true);
        }}
        style={{ margin: '10px' }}>
        Inputs settings
      </button>
      <button
        className="button button--outline button--primary"
        onClick={() => onSubmit()}
        style={{ margin: '10px' }}>
        Submit
      </button>
      <ReactModal
        isOpen={inputsSettingsModalOpen}
        onRequestClose={() => setInputsSettingsModalOpen(false)}
        overlayClassName={styles.overlay}
        className={styles.content}
        ariaHideApp={false}>
        <SettingsInputs
          closeModal={() => setInputsSettingsModalOpen(false)}
          handleSettingsUpdate={onChange}
          selectedInputResolutionNames={selectedInputResolutionNames}
        />
      </ReactModal>
    </div>
  );
}

export default PlaygroundRenderSettings;
