import { Tooltip } from 'react-tooltip';

function SubmitButton({
  onSubmit,
  readyToSubmit,
}: {
  onSubmit: () => Promise<void>;
  readyToSubmit: boolean;
}) {
  return (
    <div
      data-tooltip-id={readyToSubmit ? null : 'disableSubmit'}
      data-tooltip-content={readyToSubmit ? null : 'Enter valid JSON!'}
      data-tooltip-place={readyToSubmit ? null : 'top'}>
      <button
        className={`button ${
          readyToSubmit ? 'button--outline button--primary' : 'disabled button--secondary'
        }`}
        style={readyToSubmit ? {} : { color: '#f5f5f5', backgroundColor: '#dbdbdb' }}
        onClick={onSubmit}>
        Submit
      </button>
      <Tooltip id="disableSubmit" />
    </div>
  );
}

export default SubmitButton;
