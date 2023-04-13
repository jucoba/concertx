import React, { useId } from 'react';

import MicIcon from '../icons/Mic';
import clsx from 'clsx';
import { MagnifyingGlassIcon } from '@heroicons/react/20/solid';

type SearchProps = {
  className?: string;
};

const Search: React.FC<SearchProps> = ({ className }) => {
  const voiceSearchId = useId();
  return (
    <form className={clsx('flex items-center', className)}>
      <label htmlFor={voiceSearchId} className="sr-only">
        Search
      </label>
      <div className="relative w-full">
        <div className="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
          <MagnifyingGlassIcon className="w-5 h-5 text-gray-500 dark:text-gray-400" />
        </div>
        <input
          type="text"
          id={voiceSearchId}
          className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 pr-7 p-2.5  dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
          placeholder="Search artists, event or venue"
          required
        />
          <button type="button" className="absolute inset-y-0 right-0 flex items-center pr-3" aria-label="Click to speak your search terms">
            <MicIcon className="w-4 h-4 text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white" />
          </button>
      </div>
    </form>
  );
};

export default Search;