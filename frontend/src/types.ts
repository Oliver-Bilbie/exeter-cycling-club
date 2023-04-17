export type ResponseType = {
  message?: string;
  body?: string;
};

export type UserType = {
  id: string;
  name: string;
  access_token: string;
  admin: boolean;
};

export type RouteType = {
  id: string;
  name: string;
  distance: string;
  elevation_gain: string;
  description: string;
  gpx: string;
};

export const EMPTY_ROUTE = {
  id: "",
  name: "",
  distance: "",
  elevation_gain: "",
  description: "",
  gpx: "",
} as RouteType;
