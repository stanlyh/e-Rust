import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { clientsApi } from '../../lib/api/clients';
import { toast } from '../../lib/toast';

const schema = z.object({
  first_name: z.string().min(1, 'Requerido').max(100),
  last_name: z.string().min(1, 'Requerido').max(100),
  email: z.string().email('Email invalido').optional().or(z.literal('')),
  phone: z.string().max(30).optional(),
  mobile: z.string().max(30).optional(),
  id_document: z.string().optional(),
  address: z.string().optional(),
  city: z.string().optional(),
  notes: z.string().max(2000).optional(),
});

type FormData = z.infer<typeof schema>;

export default function ClientForm() {
  const { register, handleSubmit, formState: { errors, isSubmitting }, setError } =
    useForm<FormData>({ resolver: zodResolver(schema) });

  const onSubmit = async (data: FormData) => {
    try {
      const client = await clientsApi.create({
        ...data,
        email: data.email || undefined,
        phone: data.phone || undefined,
        mobile: data.mobile || undefined,
      });
      toast.success('Cliente creado correctamente');
      window.location.href = `/clients/${client.id}`;
    } catch (err: any) {
      const msg = err?.data?.error ?? err?.message ?? 'Error al crear el cliente.';
      toast.error(msg);
      setError('root', { message: msg });
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="max-w-lg space-y-5">
      {errors.root && (
        <div className="bg-red-50 border border-red-200 text-red-700 text-sm px-4 py-3 rounded-lg">
          {errors.root.message}
        </div>
      )}

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Nombre *</label>
          <input {...register('first_name')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" />
          {errors.first_name && <p className="text-red-500 text-xs mt-1">{errors.first_name.message}</p>}
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Apellido *</label>
          <input {...register('last_name')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" />
          {errors.last_name && <p className="text-red-500 text-xs mt-1">{errors.last_name.message}</p>}
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Email</label>
        <input {...register('email')} type="email" className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" />
        {errors.email && <p className="text-red-500 text-xs mt-1">{errors.email.message}</p>}
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Telefono</label>
          <input {...register('phone')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Celular</label>
          <input {...register('mobile')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Documento</label>
          <input {...register('id_document')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" placeholder="Cedula / RUC" />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">Ciudad</label>
          <input {...register('city')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" />
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Direccion</label>
        <input {...register('address')} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm" />
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">Notas</label>
        <textarea {...register('notes')} rows={3} className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm resize-none" />
      </div>

      <div className="flex gap-3">
        <a href="/clients" className="flex-1 text-center border border-gray-300 text-gray-700 text-sm font-medium py-2 rounded-lg hover:bg-gray-50">Cancelar</a>
        <button type="submit" disabled={isSubmitting} className="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-sm font-medium py-2 rounded-lg disabled:opacity-50">
          {isSubmitting ? 'Guardando...' : 'Crear Cliente'}
        </button>
      </div>
    </form>
  );
}
