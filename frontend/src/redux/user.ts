import { createSlice } from "@reduxjs/toolkit";
import { UserType } from "../types";

const initialState: UserType = {
  id: "",
  name: "",
  access_token: "",
  admin: false,
};

export const user = createSlice({
  name: "userData",
  initialState: initialState,
  reducers: {
    setData: (state, action): void => {
      return action.payload;
    },
  },
});

export const { setData } = user.actions;

export default user.reducer;
