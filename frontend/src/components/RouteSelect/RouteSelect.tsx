import React from "react";
import { Text, Image, DataTable } from "grommet";

interface RouteSelectProps {
  width: number;
  routes: any[];
  onSelect: (datum) => void;
}

const RouteSelect: React.FC<RouteSelectProps> = ({
  width,
  routes,
  onSelect,
}): React.ReactElement => {
  // Pixel width of the page below which the ride table will be adjusted for smaller displays
  const WIDTH_THRESHOLD = 800;

  const FULL_COLUMNS = [
    {
      property: "image",
      header: <Text>Map</Text>,
      render: ({ image }) => (
        <Image
          src={image}
          fit="contain"
          width={
            width > WIDTH_THRESHOLD
              ? width > 2000
                ? "400px"
                : `${width / 5}px`
              : "200px"
          }
        />
      ),
      width:
        width > WIDTH_THRESHOLD
          ? width > 2000
            ? "400px"
            : `${width / 5}px`
          : "200px",
    },
    {
      property: "name",
      header: <Text>Name</Text>,
      primary: true,
      search: true,
    },
    {
      property: "updated",
      header: <Text alignSelf="center">Last Updated</Text>,
      render: (datum) => (
        <Text textAlign="end">{datum.updated.replace(/T|Z/g, " ")}</Text>
      ),
    },
  ];

  const SLIM_COLUMNS = [
    {
      property: "image",
      header: <Text>Map</Text>,
      render: ({ image }) => (
        <Image
          src={image}
          fit="contain"
          width={
            width > WIDTH_THRESHOLD
              ? width > 2000
                ? "400px"
                : `${width / 5}px`
              : "200px"
          }
        />
      ),
    },
    {
      property: "name",
      header: <Text>Name</Text>,
      primary: true,
      search: true,
    },
  ];

  return (
    <>
      {routes.length > 0 ? (
        <DataTable
          columns={width > WIDTH_THRESHOLD ? FULL_COLUMNS : SLIM_COLUMNS}
          data={routes}
          onClickRow={({ datum }): void => onSelect(datum)}
          background={{ header: "background-dark" }}
        />
      ) : (
        <Text textAlign="center">
          No routes were found on your Strava account. Routes marked as private
          will not be displayed here.
        </Text>
      )}
    </>
  );
};

export default RouteSelect;
