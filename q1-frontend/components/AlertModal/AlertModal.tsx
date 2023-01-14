import { SetStateAction, Dispatch, FormEvent } from "react";
import { TableContents } from "../Table/Table";

interface AlertModalProps {
  useContents: Dispatch<SetStateAction<TableContents>>,
  contents: TableContents,
}

export default function AlertModal({useContents, contents}: AlertModalProps, ) {
  function OnSubmitEvent(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const alert = {
      alert: (e.target as any).elements[0].value,
      status: '',
      updates: []
    }
    
    let clone = {...contents};
    clone.rowContents.push(alert);
    useContents(clone);
  }

  return (
    <form data-testid='form' onSubmit={OnSubmitEvent}>
      <label> Add new alert: </label>
      <input type='text' id='alert' name='alert' />
      <button type='submit'> Add </button>
    </form>
  )
}
