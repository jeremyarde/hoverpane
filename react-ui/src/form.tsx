type WidgetSchema = {
  url: string;
  refresh_interval: number;
  level: "normal" | "alwaysontop" | "alwaysonbottom";
};

const defaultValues: WidgetSchema = {
  level: "normal",
  url: "",
  refresh_interval: 0,
};

export default function WidgetForm() {
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const data = {
      url: formData.get("url") as string,
      level: formData.get("level") as WidgetSchema["level"],
      refresh_interval: formData.get("refresh_interval")
        ? Number(formData.get("refresh_interval"))
        : 0,
    };
    console.log("Form submitted:", data);
  };

  const handleReset = (event: React.FormEvent<HTMLFormElement>) => {
    event.currentTarget.reset();
  };

  const inputClass =
    "w-full px-3 h-10 bg-[#FDFD96] border-[3px] border-black text-xl focus:outline-none appearance-none";
  const labelClass =
    "block w-full bg-[#FF90BC] h-10 leading-10 border-x-[3px] border-t-[3px] border-black text-center font-black text-xl uppercase";

  return (
    <div className="p-4 max-w-xl mx-auto">
      <div className="shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]">
        <div className="bg-[#FF90BC] h-16 flex items-center justify-center border-b-[3px] border-x-[3px] border-t-[3px] border-black">
          <h2 className="text-3xl font-black text-center uppercase tracking-[0.2em]">
            Create Widget
          </h2>
        </div>

        <form onSubmit={handleSubmit} onReset={handleReset}>
          {/* URL Field */}
          <div>
            <label htmlFor="url" className={labelClass}>
              URL
            </label>
            <input
              type="text"
              id="url"
              name="url"
              defaultValue={defaultValues.url}
              placeholder="https://example.com"
              className={`${inputClass} border-b-[3px]`}
              required
            />
          </div>

          {/* Window Level Field */}
          <div>
            <label htmlFor="level" className={labelClass}>
              Window Level
            </label>
            <select
              id="level"
              name="level"
              defaultValue={defaultValues.level}
              className={`${inputClass} border-b-[3px] cursor-pointer`}
            >
              <option value="normal">Normal</option>
              <option value="alwaysontop">Always on Top</option>
              <option value="alwaysonbottom">Always on Bottom</option>
            </select>
          </div>

          {/* Refresh Interval Field */}
          <div>
            <label htmlFor="refresh_interval" className={labelClass}>
              Refresh Interval (ms)
            </label>
            <input
              type="number"
              id="refresh_interval"
              name="refresh_interval"
              defaultValue={defaultValues.refresh_interval}
              min="0"
              placeholder="0"
              className={`${inputClass} border-b-[3px]`}
            />
          </div>

          {/* Form Actions */}
          <div className="flex justify-end bg-[#FF90BC] border-t-[3px] border-x-[3px] border-b-[3px] border-black">
            <button
              type="reset"
              className="h-12 px-8 text-xl font-black bg-[#98EECC] border-l-[3px] border-black uppercase hover:bg-[#7DCCAA] active:bg-[#98EECC] transition-colors"
            >
              Reset
            </button>
            <button
              type="submit"
              className="h-12 px-8 text-xl font-black bg-[#A7D2CB] border-l-[3px] border-black uppercase hover:bg-[#86B1AA] active:bg-[#A7D2CB] transition-colors"
            >
              Create Widget
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
