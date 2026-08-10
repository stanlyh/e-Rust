interface PaginationProps {
  page: number;
  total: number;
  perPage: number;
  onPage: (page: number) => void;
}

export function Pagination({ page, total, perPage, onPage }: PaginationProps) {
  const totalPages = Math.ceil(total / perPage);
  if (totalPages <= 1) return null;

  return (
    <div className="flex items-center justify-between px-4 py-3 border-t border-gray-100 dark:border-gray-700">
      <button
        onClick={() => onPage(page - 1)}
        disabled={page === 1}
        className="text-sm text-gray-600 dark:text-gray-400 disabled:opacity-40 hover:text-blue-600 dark:hover:text-blue-400 disabled:cursor-not-allowed"
      >
        ← Anterior
      </button>
      <span className="text-xs text-gray-500 dark:text-gray-400">
        Pagina <span className="font-medium">{page}</span> de <span className="font-medium">{totalPages}</span>
        {' '}· {total} registros
      </span>
      <button
        onClick={() => onPage(page + 1)}
        disabled={page >= totalPages}
        className="text-sm text-gray-600 dark:text-gray-400 disabled:opacity-40 hover:text-blue-600 dark:hover:text-blue-400 disabled:cursor-not-allowed"
      >
        Siguiente →
      </button>
    </div>
  );
}
