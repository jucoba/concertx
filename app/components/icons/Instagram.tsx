import React from 'react';

type IconProps = {
  className?: string;
  color?: string;
};

const Icon: React.FC<IconProps> = ({
  className,
  color = 'currentColor'
}) => {
  return (
    <svg
      aria-hidden="true"
      focusable="false"
      role="img"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill={color}
    >
      <path d="M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z" />
    </svg>
  );
};

export default Icon;

