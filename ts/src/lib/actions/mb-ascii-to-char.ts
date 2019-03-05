import { CaseStyle } from "kryo/case-style";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { LiteralType } from "kryo/types/literal";
import { ActionBase } from "../action-base";
import { $ActionType, ActionType } from "../action-type";

export interface MbAsciiToChar extends ActionBase {
  action: ActionType.MbAsciiToChar;
}

export const $MbAsciiToChar: DocumentIoType<MbAsciiToChar> = new DocumentType<MbAsciiToChar>({
  properties: {
    action: {type: new LiteralType({type: $ActionType, value: ActionType.MbAsciiToChar as ActionType.MbAsciiToChar})},
  },
  changeCase: CaseStyle.SnakeCase,
});
