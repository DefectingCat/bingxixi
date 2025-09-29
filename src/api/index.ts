// 用户信息
export interface BaseResponse<T> {
  success: boolean;
  errorCode: number;
  errorMsg: null;
  result: T;
}
export interface MMSUser {
  id: number;
  username: string;
  mobile: string;
  nickname: string;
  mallOwner: boolean;
  roleId: number;
  isBindWeChat: boolean;
  forceMobileVerify: boolean;
  mallVerify: boolean;
  mallInfoAuthority: boolean;
  conjoinedUserId: null;
  mall_id: number;
  password_status: number;
  server_time: number;
  created_at: Date;
  updated_at: Date;
}
