import { InputResolutionNames, ResolutionName } from '../Resolution';
import styles from './SettingsInputs.module.css';

interface PlaygroundSettingsInputsProps {
  closeModal: () => void;
  handleSettingsUpdate: (string, Resolution) => void;
  selectedInputResolutionNames: InputResolutionNames;
}

export default function PlaygroundRenderSettingsInputs({
  closeModal,
  handleSettingsUpdate,
  selectedInputResolutionNames,
}: PlaygroundSettingsInputsProps) {
  function handleChange(event, inputId: string) {
    handleSettingsUpdate(inputId, event.target.value);
  }

  return (
    <div className={styles.container}>
      {Object.keys(selectedInputResolutionNames).map(inputId => (
        <InputResolutionSelect
          inputName={inputId}
          selectedValue={selectedInputResolutionNames[inputId]}
          handleChange={event => handleChange(event, inputId)}
          key={inputId}
        />
      ))}
      <div className={styles.controlButtonsContainer}>
        <button
          onClick={closeModal}
          className={`${styles.controlButton} button button--block button--outline button--primary`}>
          Close
        </button>
      </div>
    </div>
  );
}

interface InputResolutionSelectProps {
  inputName: string;
  selectedValue: ResolutionName;
  handleChange: (Event) => void;
}

function InputResolutionSelect({
  inputName,
  selectedValue,
  handleChange,
}: InputResolutionSelectProps) {
  return (
    <div className={styles.inputSelector}>
      <label className={styles.inputSelector_label}>{inputName}</label>
      <select onChange={handleChange} value={selectedValue} className={styles.inputSelector_select}>
        <option value={ResolutionName.Resoultion1920x1080}>[16:9] 1920x1080</option>
        <option value={ResolutionName.Resoultion1080x1920}>[9:16] 1080x1920</option>
        <option value={ResolutionName.Resoultion854x480}>[16:9] 854x480</option>
        <option value={ResolutionName.Resoultion480x854}>[9:16] 480x854</option>
        <option value={ResolutionName.Resoultion1440x1080}>[4:3] 1440x1080</option>
        <option value={ResolutionName.Resoultion1080x1440}>[3:4] 1080x1440</option>
      </select>
    </div>
  );
}
