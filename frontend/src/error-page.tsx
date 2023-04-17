import React from "react";

export default function ErrorPage(): JSX.Element {
  return (
    <div id="error-page">
      <h1>Oops!</h1>
      <p>An unexpected error has occurred.</p>
      <p>
        Until this is resolved, please{" "}
        <a href="https://www.facebook.com/Exeter-and-East-Devon-Cycling-Group-2263893193890118/">
          visit our Facebook page
        </a>{" "}
        instead.
      </p>
    </div>
  );
}
